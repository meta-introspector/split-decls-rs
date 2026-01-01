// SRC: ../rust/compiler/rustc_attr_parsing/src/attributes/confusables.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */
use crate::prelude::*;
use crate::session_diagnostics::EmptyConfusables;

#[derive(Default)]
pub(crate) struct ConfusablesParser {
    confusables: ThinVec<Symbol>,
    first_span: Option<Span>,
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=finalize | COMPLEXITY=21 | LINES=43 */

impl<S: Stage> AttributeParser<S> for ConfusablesParser {
    const ATTRIBUTES: AcceptMapping<Self, S> = &[(
        &[sym::rustc_confusables],
        template!(List: &[r#""name1", "name2", ..."#]),
        |this, cx, args| {
            let Some(list) = args.list() else {
                cx.expected_list(cx.attr_span);
                return;
            };

            if list.is_empty() {
                cx.emit_err(EmptyConfusables { span: cx.attr_span });
            }

            for param in list.mixed() {
                let span = param.span();

                let Some(lit) = param.lit().and_then(|i| i.value_str()) else {
                    cx.expected_string_literal(span, param.lit());
                    continue;
                };

                this.confusables.push(lit);
            }

            this.first_span.get_or_insert(cx.attr_span);
        },
    )];
    const ALLOWED_TARGETS: AllowedTargets =
        AllowedTargets::AllowList(&[Allow(Target::Method(MethodKind::Inherent))]);

    fn finalize(self, _cx: &FinalizeContext<'_, '_, S>) -> Option<AttributeKind> {
        if self.confusables.is_empty() {
            return None;
        }

        Some(AttributeKind::Confusables {
            symbols: self.confusables,
            first_span: self.first_span.unwrap(),
        })
    }
}