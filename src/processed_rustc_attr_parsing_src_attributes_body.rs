// SRC: ../rust/compiler/rustc_attr_parsing/src/attributes/body.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=12 */
// Attributes that can be found in function body.

use crate::prelude::*;

pub(crate) struct CoroutineParser;

impl<S: Stage> NoArgsAttributeParser<S> for CoroutineParser {
    const PATH: &[Symbol] = &[sym::coroutine];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Error;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Closure)]);
    const CREATE: fn(crate::rustc_span::Span) -> AttributeKind = |span| AttributeKind::Coroutine(span);
}