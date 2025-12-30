// Generated macro for impl_1270 (impl)
macro_rules! Depcrate_runtime_selimpl_1270 {
() => {
// Module: crate::runtime::sel
// Provides: {"impl_1270"}
// Dependencies: {}
impl PartialEq for Sel { # [inline] fn eq (& self , other : & Self) -> bool { if cfg ! (feature = "gnustep-1-7") { unsafe { ffi :: sel_isEqual (* self , * other) . as_bool () } } else { ptr :: eq (self . as_ptr () , other . as_ptr ()) } } }
};
}
