// Generated macro for impl_147 (impl)
macro_rules! Depcrateimpl_147 {
() => {
// Module: crate
// Provides: {"impl_147"}
// Dependencies: {}
impl Drop for DropDealloc { # [inline] fn drop (& mut self) { unsafe { if self . size_bytes > 0 { alloc :: alloc :: dealloc (self . ptr . as_ptr () , Layout :: from_size_align_unchecked (self . size_bytes , self . align) ,) ; } } } }
};
}
