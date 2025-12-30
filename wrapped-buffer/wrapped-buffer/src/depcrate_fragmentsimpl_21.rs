// Generated macro for impl_21 (impl)
macro_rules! Depcrate_fragmentsimpl_21 {
() => {
// Module: crate::fragments
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'sval > fmt :: Write for TextBuf < 'sval > { fn write_str (& mut self , s : & str) -> fmt :: Result { self . push_fragment_computed (s) . map_err (| _ | fmt :: Error) } }
};
}
