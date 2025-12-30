// Generated macro for impl_111 (impl)
macro_rules! Depcrate_fmtimpl_111 {
() => {
// Module: crate::fmt
// Provides: {"impl_111"}
// Dependencies: {}
impl ConfigurableFormat { # [doc = " Format the [`Record`] as configured for outputting"] pub fn format (& self , formatter : & mut Formatter , record : & Record < '_ >) -> io :: Result < () > { let fmt = ConfigurableFormatWriter { format : self , buf : formatter , written_header_value : false , } ; fmt . write (record) } }
};
}
