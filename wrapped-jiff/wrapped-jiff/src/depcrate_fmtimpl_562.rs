// Generated macro for impl_562 (impl)
macro_rules! Depcrate_fmtimpl_562 {
() => {
// Module: crate::fmt
// Provides: {"impl_562"}
// Dependencies: {}
impl < W : Write > core :: fmt :: Write for StdFmtWrite < W > { # [inline] fn write_str (& mut self , string : & str) -> Result < () , core :: fmt :: Error > { self . 0 . write_str (string) . map_err (| _ | core :: fmt :: Error) } }
};
}
