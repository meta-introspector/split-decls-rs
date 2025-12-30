// Generated macro for impl_1887 (impl)
macro_rules! Depcrate_syscallsimpl_1887 {
() => {
// Module: crate::syscalls
// Provides: {"impl_1887"}
// Dependencies: {}
impl Dirent64 { # [doc = " Creates a [`Dirent64Display`] struct for debug printing."] # [doc = ""] # [doc = " # Safety"] # [doc = " The bytes following the `d_name` must form a valid zero terminated `CStr`. Else we have an"] # [doc = " out-of-bounds read."] # [allow (dead_code)] unsafe fn display < 'a > (& 'a self) -> Dirent64Display < 'a > { unsafe { Dirent64Display :: new (self) } } }
};
}
