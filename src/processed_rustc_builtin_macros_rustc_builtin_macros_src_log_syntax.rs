/* FP:log_syntax.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_log_syntax_USE_0001
/* FP:log_syntax.rs-0002 */ use crate :: rustc_complete :: tokenstream :: TokenStream ;
/* FP:log_syntax.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_log_syntax_USE_0002
/* FP:log_syntax.rs-0004 */ use rustc_ast_pretty :: pprust ;
/* FP:log_syntax.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_log_syntax_USE_0003
/* FP:log_syntax.rs-0006 */ use crate :: rustc_expand :: base :: { DummyResult , ExpandResult , ExtCtxt , MacroExpanderResult } ;
/* FP:log_syntax.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_log_syntax_FN_0004
/* FP:log_syntax.rs-0008 */ pub (crate) fn expand_log_syntax < 'cx > (_cx : & 'cx mut ExtCtxt < '_ > , sp : crate :: rustc_span :: Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { println ! ("{}" , pprust :: tts_to_string (& tts)) ; ExpandResult :: Ready (DummyResult :: any_valid (sp)) }