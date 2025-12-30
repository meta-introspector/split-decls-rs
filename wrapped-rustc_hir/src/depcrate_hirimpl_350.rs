// Generated macro for impl_350 (impl)
macro_rules! Depcrate_hirimpl_350 {
() => {
// Module: crate::hir
// Provides: {"impl_350"}
// Dependencies: {}
impl < 'hir > AssocItemConstraint < 'hir > { # [doc = " Obtain the type on the RHS of an assoc ty equality constraint if applicable."] pub fn ty (self) -> Option < & 'hir Ty < 'hir > > { match self . kind { AssocItemConstraintKind :: Equality { term : Term :: Ty (ty) } => Some (ty) , _ => None , } } # [doc = " Obtain the const on the RHS of an assoc const equality constraint if applicable."] pub fn ct (self) -> Option < & 'hir ConstArg < 'hir > > { match self . kind { AssocItemConstraintKind :: Equality { term : Term :: Const (ct) } => Some (ct) , _ => None , } } }
};
}
