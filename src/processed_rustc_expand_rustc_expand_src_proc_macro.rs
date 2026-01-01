// SRC: ../rust/compiler/rustc_expand/src/proc_macro.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use crate::rustc_complete::tokenstream::TokenStream;
use crate::rustc_complete::ErrorGuaranteed;
use crate::rustc_parse::parser::{ForceCollect, Parser};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
use crate::rustc_complete::config::ProcMacroExecutionStrategy;
use crate::rustc_complete::Span;
use crate::rustc_complete::profiling::SpannedEventArgRecorder;
use {rustc_ast as ast, rustc_proc_macro as pm};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

use crate::base::{self, *};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::{errors, proc_macro_server};
/* AST_META: AST_ID=5 | TYPE=STRUCT | NAME=MessagePipe | COMPLEXITY=2 | LINES=5 */

struct MessagePipe<T> {
    tx: std::sync::mpsc::SyncSender<T>,
    rx: std::sync::mpsc::Receiver<T>,
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=new | COMPLEXITY=9 | LINES=16 */

impl<T> pm::bridge::server::MessagePipe<T> for MessagePipe<T> {
    fn new() -> (Self, Self) {
        let (tx1, rx1) = std::sync::mpsc::sync_channel(1);
        let (tx2, rx2) = std::sync::mpsc::sync_channel(1);
        (MessagePipe { tx: tx1, rx: rx2 }, MessagePipe { tx: tx2, rx: rx1 })
    }

    fn send(&mut self, value: T) {
        self.tx.send(value).unwrap();
    }

    fn recv(&mut self) -> Option<T> {
        self.rx.recv().ok()
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=exec_strategy | COMPLEXITY=2 | LINES=7 */

fn exec_strategy(ecx: &ExtCtxt<'_>) -> impl pm::bridge::server::ExecutionStrategy + 'static {
    pm::bridge::server::MaybeCrossThread::<MessagePipe<_>>::new(
        ecx.sess.opts.unstable_opts.proc_macro_execution_strategy
            == ProcMacroExecutionStrategy::CrossThread,
    )
}
/* AST_META: AST_ID=8 | TYPE=STRUCT | NAME=BangProcMacro | COMPLEXITY=2 | LINES=4 */

pub struct BangProcMacro {
    pub client: pm::bridge::client::Client<pm::TokenStream, pm::TokenStream>,
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=expand | COMPLEXITY=10 | LINES=26 */

impl base::BangProcMacro for BangProcMacro {
    fn expand(
        &self,
        ecx: &mut ExtCtxt<'_>,
        span: Span,
        input: TokenStream,
    ) -> Result<TokenStream, ErrorGuaranteed> {
        let _timer =
            ecx.sess.prof.generic_activity_with_arg_recorder("expand_proc_macro", |recorder| {
                recorder.record_arg_with_span(ecx.sess.source_map(), ecx.expansion_descr(), span);
            });

        let proc_macro_backtrace = ecx.ecfg.proc_macro_backtrace;
        let strategy = exec_strategy(ecx);
        let server = proc_macro_server::Rustc::new(ecx);
        self.client.run(&strategy, server, input, proc_macro_backtrace).map_err(|e| {
            ecx.dcx().emit_err(errors::ProcMacroPanicked {
                span,
                message: e
                    .as_str()
                    .map(|message| errors::ProcMacroPanickedHelp { message: message.into() }),
            })
        })
    }
}
/* AST_META: AST_ID=10 | TYPE=STRUCT | NAME=AttrProcMacro | COMPLEXITY=2 | LINES=4 */

pub struct AttrProcMacro {
    pub client: pm::bridge::client::Client<(pm::TokenStream, pm::TokenStream), pm::TokenStream>,
}
/* AST_META: AST_ID=11 | TYPE=FUNCTION | NAME=expand | COMPLEXITY=11 | LINES=29 */

impl base::AttrProcMacro for AttrProcMacro {
    fn expand(
        &self,
        ecx: &mut ExtCtxt<'_>,
        span: Span,
        annotation: TokenStream,
        annotated: TokenStream,
    ) -> Result<TokenStream, ErrorGuaranteed> {
        let _timer =
            ecx.sess.prof.generic_activity_with_arg_recorder("expand_proc_macro", |recorder| {
                recorder.record_arg_with_span(ecx.sess.source_map(), ecx.expansion_descr(), span);
            });

        let proc_macro_backtrace = ecx.ecfg.proc_macro_backtrace;
        let strategy = exec_strategy(ecx);
        let server = proc_macro_server::Rustc::new(ecx);
        self.client.run(&strategy, server, annotation, annotated, proc_macro_backtrace).map_err(
            |e| {
                ecx.dcx().emit_err(errors::CustomAttributePanicked {
                    span,
                    message: e.as_str().map(|message| errors::CustomAttributePanickedHelp {
                        message: message.into(),
                    }),
                })
            },
        )
    }
}
/* AST_META: AST_ID=12 | TYPE=STRUCT | NAME=DeriveProcMacro | COMPLEXITY=2 | LINES=4 */

pub struct DeriveProcMacro {
    pub client: pm::bridge::client::Client<pm::TokenStream, pm::TokenStream>,
}
/* AST_META: AST_ID=13 | TYPE=FUNCTION | NAME=expand | COMPLEXITY=47 | LINES=77 */

impl MultiItemModifier for DeriveProcMacro {
    fn expand(
        &self,
        ecx: &mut ExtCtxt<'_>,
        span: Span,
        _meta_item: &ast::MetaItem,
        item: Annotatable,
        _is_derive_const: bool,
    ) -> ExpandResult<Vec<Annotatable>, Annotatable> {
        // We need special handling for statement items
        // (e.g. `fn foo() { #[derive(Debug)] struct Bar; }`)
        let is_stmt = matches!(item, Annotatable::Stmt(..));

        // We used to have an alternative behaviour for crates that needed it.
        // We had a lint for a long time, but now we just emit a hard error.
        // Eventually we might remove the special case hard error check
        // altogether. See #73345.
        crate::base::ann_pretty_printing_compatibility_hack(&item, &ecx.sess.psess);
        let input = item.to_tokens();
        let stream = {
            let _timer =
                ecx.sess.prof.generic_activity_with_arg_recorder("expand_proc_macro", |recorder| {
                    recorder.record_arg_with_span(
                        ecx.sess.source_map(),
                        ecx.expansion_descr(),
                        span,
                    );
                });
            let proc_macro_backtrace = ecx.ecfg.proc_macro_backtrace;
            let strategy = exec_strategy(ecx);
            let server = proc_macro_server::Rustc::new(ecx);
            match self.client.run(&strategy, server, input, proc_macro_backtrace) {
                Ok(stream) => stream,
                Err(e) => {
                    ecx.dcx().emit_err({
                        errors::ProcMacroDerivePanicked {
                            span,
                            message: e.as_str().map(|message| {
                                errors::ProcMacroDerivePanickedHelp { message: message.into() }
                            }),
                        }
                    });
                    return ExpandResult::Ready(vec![]);
                }
            }
        };

        let error_count_before = ecx.dcx().err_count();
        let mut parser = Parser::new(&ecx.sess.psess, stream, Some("proc-macro derive"));
        let mut items = vec![];

        loop {
            match parser.parse_item(ForceCollect::No) {
                Ok(None) => break,
                Ok(Some(item)) => {
                    if is_stmt {
                        items.push(Annotatable::Stmt(Box::new(ecx.stmt_item(span, item))));
                    } else {
                        items.push(Annotatable::Item(item));
                    }
                }
                Err(err) => {
                    err.emit();
                    break;
                }
            }
        }

        // fail if there have been errors emitted
        if ecx.dcx().err_count() > error_count_before {
            ecx.dcx().emit_err(errors::ProcMacroDeriveTokens { span });
        }

        ExpandResult::Ready(items)
    }
}