// Generated macro for impl_559 (impl)
macro_rules! Depcrate_fmtimpl_559 {
() => {
// Module: crate::fmt
// Provides: {"impl_559"}
// Dependencies: {}
# [cfg (feature = "std")] impl < W : std :: io :: Write > Write for StdIoWrite < W > { # [inline] fn write_str (& mut self , string : & str) -> Result < () , Error > { self . 0 . write_all (string . as_bytes ()) . map_err (Error :: adhoc) } }
};
}
