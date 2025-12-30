// Generated macro for impl_94 (impl)
macro_rules! Depcrate_inferimpl_94 {
() => {
// Module: crate::infer
// Provides: {"impl_94"}
// Dependencies: {}
impl < 'db > Adjustment < 'db > { pub fn borrow (interner : DbInterner < 'db > , m : Mutability , ty : Ty < 'db > , lt : Region < 'db >) -> Self { let ty = Ty :: new_ref (interner , lt , ty , m) ; Adjustment { kind : Adjust :: Borrow (AutoBorrow :: Ref (AutoBorrowMutability :: new (m , AllowTwoPhase :: No))) , target : ty , } } }
};
}
