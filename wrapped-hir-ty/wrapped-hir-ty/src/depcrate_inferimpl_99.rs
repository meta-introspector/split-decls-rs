// Generated macro for impl_99 (impl)
macro_rules! Depcrate_inferimpl_99 {
() => {
// Module: crate::infer
// Provides: {"impl_99"}
// Dependencies: {}
impl AutoBorrowMutability { # [doc = " Creates an `AutoBorrowMutability` from a mutability and allowance of two phase borrows."] # [doc = ""] # [doc = " Note that when `mutbl.is_not()`, `allow_two_phase_borrow` is ignored"] pub fn new (mutbl : Mutability , allow_two_phase_borrow : AllowTwoPhase) -> Self { match mutbl { Mutability :: Not => Self :: Not , Mutability :: Mut => Self :: Mut { allow_two_phase_borrow } , } } }
};
}
