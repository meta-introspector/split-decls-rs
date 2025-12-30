// Generated macro for WhereEqPredicate (struct)
macro_rules! Depcrate_hirWhereEqPredicate {
() => {
// Module: crate::hir
// Provides: {"WhereEqPredicate"}
// Dependencies: {}
# [doc = " An equality predicate (e.g., `T = int`); currently unsupported."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct WhereEqPredicate < 'hir > { pub lhs_ty : & 'hir Ty < 'hir > , pub rhs_ty : & 'hir Ty < 'hir > , }
};
}
