// SRC: ../rust/compiler/rustc_attr_parsing/src/attributes/prelude.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
// data structures
#[doc(hidden)]
pub(super) use crate::rustc_feature::{AttributeTemplate, AttributeType, template};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */
#[doc(hidden)]
pub(super) use crate::rustc_complete::attrs::AttributeKind;
#[doc(hidden)]
pub(super) use crate::rustc_complete::lints::AttributeLintKind;
#[doc(hidden)]
pub(super) use crate::rustc_complete::{MethodKind, Target};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
#[doc(hidden)]
pub(super) use crate::rustc_complete::{DUMMY_SP, Ident, Span, Symbol, sym};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */
#[doc(hidden)]
pub(super) use thin_vec::ThinVec;

#[doc(hidden)]
pub(super) use crate::attributes::{
    AcceptMapping, AttributeOrder, AttributeParser, CombineAttributeParser, ConvertFn,
    NoArgsAttributeParser, OnDuplicate, SingleAttributeParser,
};
/* AST_META: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
// contexts
#[doc(hidden)]
pub(super) use crate::context::{AcceptContext, FinalizeContext, Stage};
/* AST_META: AST_ID=6 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */
#[doc(hidden)]
pub(super) use crate::parser::*;
// target checking
#[doc(hidden)]
pub(super) use crate::target_checking::Policy::{Allow, Error, Warn};
/* AST_META: AST_ID=7 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
#[doc(hidden)]
pub(super) use crate::target_checking::{ALL_TARGETS, AllowedTargets};