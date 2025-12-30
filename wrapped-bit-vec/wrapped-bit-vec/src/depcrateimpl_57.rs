// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
impl < B : BitBlock > Drop for MutBorrowedBit < '_ , B > { fn drop (& mut self) { let mut vec = (* self . vec) . borrow_mut () ; # [cfg (debug_assertions)] debug_assert_eq ! (Some (self . old_value) , vec . get (self . index) , "Mutably-borrowed bit was modified externally!") ; vec . set (self . index , self . new_value) ; } }
};
}
