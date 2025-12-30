// Generated macro for PatExprKind (enum)
macro_rules! Depcrate_hirPatExprKind {
() => {
// Module: crate::hir
// Provides: {"PatExprKind"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum PatExprKind < 'hir > { Lit { lit : Lit , negated : bool , } , ConstBlock (ConstBlock) , # [doc = " A path pattern for a unit struct/variant or a (maybe-associated) constant."] Path (QPath < 'hir >) , }
};
}
