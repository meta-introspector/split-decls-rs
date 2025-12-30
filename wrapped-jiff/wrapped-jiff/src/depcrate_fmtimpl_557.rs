// Generated macro for impl_557 (impl)
macro_rules! Depcrate_fmtimpl_557 {
() => {
// Module: crate::fmt
// Provides: {"impl_557"}
// Dependencies: {}
impl < W : Write > Write for & mut W { fn write_str (& mut self , string : & str) -> Result < () , Error > { (* * self) . write_str (string) } # [inline] fn write_char (& mut self , char : char) -> Result < () , Error > { (* * self) . write_char (char) } }
};
}
