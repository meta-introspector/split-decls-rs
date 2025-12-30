// Generated macro for create_dummy_fd (function)
macro_rules! Depcrate_tests_cancelcreate_dummy_fd {
() => {
// Module: crate::tests::cancel
// Provides: {"create_dummy_fd"}
// Dependencies: {}
fn create_dummy_fd () -> anyhow :: Result < File > { unsafe { let fd = libc :: eventfd (0 , libc :: EFD_CLOEXEC) ; if fd == - 1 { return Err (std :: io :: Error :: last_os_error () . into ()) ; } Ok (File :: from_raw_fd (fd)) } }
};
}
