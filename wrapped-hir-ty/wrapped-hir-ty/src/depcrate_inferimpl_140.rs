// Generated macro for impl_140 (impl)
macro_rules! Depcrate_inferimpl_140 {
() => {
// Module: crate::infer
// Provides: {"impl_140"}
// Dependencies: {}
impl Adjustment { pub fn borrow (m : Mutability , ty : Ty , lt : Lifetime) -> Self { let ty = TyKind :: Ref (m , lt . clone () , ty) . intern (Interner) ; Adjustment { kind : Adjust :: Borrow (AutoBorrow :: Ref (lt , m)) , target : ty } } }
};
}
