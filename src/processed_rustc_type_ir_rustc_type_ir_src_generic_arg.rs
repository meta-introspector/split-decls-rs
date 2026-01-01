// SRC: ../rust/compiler/rustc_type_ir/src/generic_arg.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use derive_where::derive_where;
#[cfg(feature = "nightly")]
use rustc_macros::{Decodable_NoContext, Encodable_NoContext, HashStable_NoContext};
/* AST_META: AST_ID=2 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=13 */

use crate::Interner;

#[derive_where(Clone, Copy, PartialEq, Debug; I: Interner)]
#[cfg_attr(
    feature = "nightly",
    derive(Decodable_NoContext, Encodable_NoContext, HashStable_NoContext)
)]
pub enum GenericArgKind<I: Interner> {
    Lifetime(I::Region),
    Type(I::Ty),
    Const(I::Const),
}
/* AST_META: AST_ID=3 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl<I: Interner> Eq for GenericArgKind<I> {}
/* AST_META: AST_ID=4 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive_where(Clone, Copy, PartialEq, Debug; I: Interner)]
#[cfg_attr(
    feature = "nightly",
    derive(Decodable_NoContext, Encodable_NoContext, HashStable_NoContext)
)]
pub enum TermKind<I: Interner> {
    Ty(I::Ty),
    Const(I::Const),
}
/* AST_META: AST_ID=5 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl<I: Interner> Eq for TermKind<I> {}