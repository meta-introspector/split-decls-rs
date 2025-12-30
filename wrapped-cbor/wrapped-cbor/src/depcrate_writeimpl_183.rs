// Generated macro for impl_183 (impl)
macro_rules! Depcrate_writeimpl_183 {
() => {
// Module: crate::write
// Provides: {"impl_183"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl < 'a , W : Write > fmt :: Write for FmtWrite < 'a , W > { fn write_str (& mut self , s : & str) -> fmt :: Result { self . 0 . write_all (s . as_bytes ()) . map_err (| _ | fmt :: Error) } }
};
}
