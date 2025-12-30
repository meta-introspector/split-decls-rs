// Generated macro for init_root (function)
macro_rules! Depcrate_pathsinit_root {
() => {
// Module: crate::paths
// Provides: {"init_root"}
// Dependencies: {}
# [doc = " For test harnesses like [`crate::cargo_test`]"] pub fn init_root (tmp_dir : & 'static str) -> TestIdGuard { static NEXT_ID : AtomicUsize = AtomicUsize :: new (0) ; let id = NEXT_ID . fetch_add (1 , Ordering :: SeqCst) ; TEST_ID . with (| n | * n . borrow_mut () = Some (id)) ; let guard = TestIdGuard { _private : () } ; set_global_root (tmp_dir) ; let r = root () ; r . rm_rf () ; r . mkdir_p () ; guard }
};
}
