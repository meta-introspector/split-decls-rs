// Generated macro for impl_126 (impl)
macro_rules! Depcrate_writersimpl_126 {
() => {
// Module: crate::writers
// Provides: {"impl_126"}
// Dependencies: {}
# [cfg (feature = "issue-url")] impl < W > fmt :: Write for FooterWriter < W > where W : fmt :: Write , { fn write_str (& mut self , s : & str) -> fmt :: Result { if ! self . had_output && ! s . is_empty () { self . had_output = true ; } self . inner . write_str (s) } }
};
}
