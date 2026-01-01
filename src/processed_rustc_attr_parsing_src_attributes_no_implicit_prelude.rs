// SRC: ../rust/compiler/rustc_attr_parsing/src/attributes/no_implicit_prelude.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=11 */
use crate::prelude::*;

pub(crate) struct NoImplicitPreludeParser;

impl<S: Stage> NoArgsAttributeParser<S> for NoImplicitPreludeParser {
    const PATH: &[crate::rustc_span::Symbol] = &[sym::no_implicit_prelude];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Warn;
    const ALLOWED_TARGETS: AllowedTargets =
        AllowedTargets::AllowListWarnRest(&[Allow(Target::Mod), Allow(Target::Crate)]);
    const CREATE: fn(Span) -> AttributeKind = AttributeKind::NoImplicitPrelude;
}