// Generated macro for impl_126 (impl)
macro_rules! Depcrate_rawimpl_126 {
() => {
// Module: crate::raw
// Provides: {"impl_126"}
// Dependencies: {}
impl < T , A : Allocator > Drop for RawDrain < '_ , T , A > { # [cfg_attr (feature = "inline-more" , inline)] fn drop (& mut self) { unsafe { self . iter . drop_elements () ; self . table . clear_no_drop () ; self . orig_table . as_ptr () . copy_from_nonoverlapping (& self . table , 1) ; } } }
};
}
