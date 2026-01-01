// SRC: ../rust/compiler/rustc_middle/src/middle/resolve_bound_vars.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=6 */
// Name resolution for lifetimes and late-bound type and const variables: type declarations.

use crate::rustc_data_structures::sorted_map::SortedMap;
use crate::rustc_complete::ErrorGuaranteed;
use crate::rustc_complete::ItemLocalId;
use crate::rustc_complete::def_id::{DefId, LocalDefId, LocalDefIdMap};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use rustc_macros::{Decodable, Encodable, HashStable, TyDecodable, TyEncodable};
/* AST_META: AST_ID=3 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=11 */

use crate::ty;

#[derive(Clone, Copy, PartialEq, Eq, Hash, TyEncodable, TyDecodable, Debug, HashStable)]
pub enum ResolvedArg {
    StaticLifetime,
    EarlyBound(/* decl */ LocalDefId),
    LateBound(ty::DebruijnIndex, /* late-bound index */ u32, /* decl */ LocalDefId),
    Free(LocalDefId, /* lifetime decl */ LocalDefId),
    Error(ErrorGuaranteed),
}
/* AST_META: AST_ID=4 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

/// A set containing, at most, one known element.
/// If two distinct values are inserted into a set, then it
/// becomes `Many`, which can be used to detect ambiguities.
#[derive(Copy, Clone, PartialEq, Eq, TyEncodable, TyDecodable, Debug, HashStable)]
pub enum Set1<T> {
    Empty,
    One(T),
    Many,
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=insert | COMPLEXITY=9 | LINES=10 */

impl<T: PartialEq> Set1<T> {
    pub fn insert(&mut self, value: T) {
        *self = match self {
            Set1::Empty => Set1::One(value),
            Set1::One(old) if *old == value => return,
            _ => Set1::Many,
        };
    }
}
/* AST_META: AST_ID=6 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

#[derive(Copy, Clone, Debug, HashStable, Encodable, Decodable)]
pub enum ObjectLifetimeDefault {
    Empty,
    Static,
    Ambiguous,
    Param(DefId),
}
/* AST_META: AST_ID=7 | TYPE=STRUCT | NAME=ResolveBoundVars | COMPLEXITY=5 | LINES=20 */

/// Maps the id of each bound variable reference to the variable decl
/// that it corresponds to.
#[derive(Debug, Default, HashStable)]
pub struct ResolveBoundVars {
    // Maps from every use of a named (not anonymous) bound var to a
    // `ResolvedArg` describing how that variable is bound.
    pub defs: SortedMap<ItemLocalId, ResolvedArg>,

    // Maps relevant hir items to the bound vars on them. These include:
    // - function defs
    // - function pointers
    // - closures
    // - trait refs
    // - bound types (like `T` in `for<'a> T<'a>: Foo`)
    pub late_bound_vars: SortedMap<ItemLocalId, Vec<ty::BoundVariableKind>>,

    // List captured variables for each opaque type.
    pub opaque_captured_lifetimes: LocalDefIdMap<Vec<(ResolvedArg, LocalDefId)>>,
}