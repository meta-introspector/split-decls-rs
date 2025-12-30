// Generated macro for AnonymousMmap (struct)
macro_rules! Depcrate_tests_register_buf_ringAnonymousMmap {
() => {
// Module: crate::tests::register_buf_ring
// Provides: {"AnonymousMmap"}
// Dependencies: {}
# [doc = " An anonymous region of memory mapped using `mmap(2)`, not backed by a file"] # [doc = " but that is guaranteed to be page-aligned and zero-filled."] pub struct AnonymousMmap { addr : ptr :: NonNull < libc :: c_void > , len : usize , }
};
}
