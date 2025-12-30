// Generated macro for impl_561 (impl)
macro_rules! Depcrate_fmtimpl_561 {
() => {
// Module: crate::fmt
// Provides: {"impl_561"}
// Dependencies: {}
impl < W : core :: fmt :: Write > Write for StdFmtWrite < W > { # [inline] fn write_str (& mut self , string : & str) -> Result < () , Error > { self . 0 . write_str (string) . map_err (| _ | err ! ("an error occurred when formatting an argument")) } }
};
}
