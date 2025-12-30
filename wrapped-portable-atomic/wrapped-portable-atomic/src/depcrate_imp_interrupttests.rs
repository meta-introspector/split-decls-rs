// Generated macro for tests (module)
macro_rules! Depcrate_imp_interrupttests {
() => {
// Module: crate::imp::interrupt
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; test_atomic_ptr_single_thread ! () ; test_atomic_int_single_thread ! (i8) ; test_atomic_int_single_thread ! (u8) ; test_atomic_int_single_thread ! (i16) ; test_atomic_int_single_thread ! (u16) ; test_atomic_int_single_thread ! (i32) ; test_atomic_int_single_thread ! (u32) ; test_atomic_int_single_thread ! (i64) ; test_atomic_int_single_thread ! (u64) ; test_atomic_int_single_thread ! (i128) ; test_atomic_int_single_thread ! (u128) ; test_atomic_int_single_thread ! (isize) ; test_atomic_int_single_thread ! (usize) ; }
};
}
