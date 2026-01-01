// SRC: ../rust/compiler/rustc_type_ir/src/const_kind.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */
use std::fmt;

use derive_where::derive_where;
#[cfg(feature = "nightly")]
use crate::rustc_data_structures::stable_hasher::{HashStable, StableHasher};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
#[cfg(feature = "nightly")]
use rustc_macros::{Decodable_NoContext, Encodable_NoContext, HashStable_NoContext};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use rustc_type_ir_macros::{Lift_Generic, TypeFoldable_Generic, TypeVisitable_Generic};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

use crate::{self as ty, DebruijnIndex, Interner};
/* AST_META: AST_ID=5 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=6 | LINES=36 */

/// Represents a constant in Rust.
#[derive_where(Clone, Copy, Hash, PartialEq; I: Interner)]
#[cfg_attr(
    feature = "nightly",
    derive(Encodable_NoContext, Decodable_NoContext, HashStable_NoContext)
)]
pub enum ConstKind<I: Interner> {
    /// A const generic parameter.
    Param(I::ParamConst),

    /// Infer the value of the const.
    Infer(InferConst),

    /// Bound const variable, used only when preparing a trait query.
    Bound(DebruijnIndex, I::BoundConst),

    /// A placeholder const - universally quantified higher-ranked const.
    Placeholder(I::PlaceholderConst),

    /// An unnormalized const item such as an anon const or assoc const or free const item.
    /// Right now anything other than anon consts does not actually work properly but this
    /// should
    Unevaluated(ty::UnevaluatedConst<I>),

    /// Used to hold computed value.
    Value(I::ValueConst),

    /// A placeholder for a const which could not be computed; this is
    /// propagated to avoid useless error messages.
    Error(I::ErrorGuaranteed),

    /// Unevaluated non-const-item, used by `feature(generic_const_exprs)` to represent
    /// const arguments such as `N + 1` or `foo(N)`
    Expr(I::ExprConst),
}
/* AST_META: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl<I: Interner> Eq for ConstKind<I> {}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=18 | LINES=17 */

impl<I: Interner> fmt::Debug for ConstKind<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ConstKind::*;

        match self {
            Param(param) => write!(f, "{param:?}"),
            Infer(var) => write!(f, "{var:?}"),
            Bound(debruijn, var) => crate::debug_bound_var(f, *debruijn, var),
            Placeholder(placeholder) => write!(f, "{placeholder:?}"),
            Unevaluated(uv) => write!(f, "{uv:?}"),
            Value(val) => write!(f, "{val:?}"),
            Error(_) => write!(f, "{{const error}}"),
            Expr(expr) => write!(f, "{expr:?}"),
        }
    }
}
/* AST_META: AST_ID=8 | TYPE=STRUCT | NAME=UnevaluatedConst | COMPLEXITY=2 | LINES=12 */

/// An unevaluated (potentially generic) constant used in the type-system.
#[derive_where(Clone, Copy, Debug, Hash, PartialEq; I: Interner)]
#[derive(TypeVisitable_Generic, TypeFoldable_Generic, Lift_Generic)]
#[cfg_attr(
    feature = "nightly",
    derive(Decodable_NoContext, Encodable_NoContext, HashStable_NoContext)
)]
pub struct UnevaluatedConst<I: Interner> {
    pub def: I::DefId,
    pub args: I::GenericArgs,
}
/* AST_META: AST_ID=9 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl<I: Interner> Eq for UnevaluatedConst<I> {}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=7 */

impl<I: Interner> UnevaluatedConst<I> {
    #[inline]
    pub fn new(def: I::DefId, args: I::GenericArgs) -> UnevaluatedConst<I> {
        UnevaluatedConst { def, args }
    }
}
/* AST_META: AST_ID=11 | TYPE=STRUCT | NAME=ConstVid | COMPLEXITY=4 | LINES=9 */

crate::rustc_index::newtype_index! {
    /// A **`const`** **v**ariable **ID**.
    #[encodable]
    #[orderable]
    #[debug_format = "?{}c"]
    #[gate_rustc_only]
    pub struct ConstVid {}
}
/* AST_META: AST_ID=12 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=8 | LINES=10 */

/// An inference variable for a const, for use in const generics.
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "nightly", derive(Encodable_NoContext, Decodable_NoContext))]
pub enum InferConst {
    /// Infer the value of the const.
    Var(ConstVid),
    /// A fresh const variable. See `infer::freshen` for more details.
    Fresh(u32),
}
/* AST_META: AST_ID=13 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=11 | LINES=9 */

impl fmt::Debug for InferConst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InferConst::Var(var) => write!(f, "{var:?}"),
            InferConst::Fresh(var) => write!(f, "Fresh({var:?})"),
        }
    }
}
/* AST_META: AST_ID=14 | TYPE=FUNCTION | NAME=hash_stable | COMPLEXITY=11 | LINES=12 */

#[cfg(feature = "nightly")]
impl<CTX> HashStable<CTX> for InferConst {
    fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher) {
        match self {
            InferConst::Var(_) => {
                panic!("const variables should not be hashed: {self:?}")
            }
            InferConst::Fresh(i) => i.hash_stable(hcx, hasher),
        }
    }
}