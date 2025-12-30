// Generated macro for impl_28 (impl)
macro_rules! Depcrate_frameimpl_28 {
() => {
// Module: crate::frame
// Provides: {"impl_28"}
// Dependencies: {}
impl < R : gimli :: Reader > FunctionName < R > { # [doc = " The raw name of this function before demangling."] pub fn raw_name (& self) -> Result < Cow < '_ , str > , Error > { self . name . to_string_lossy () } # [doc = " The name of this function after demangling (if applicable)."] pub fn demangle (& self) -> Result < Cow < '_ , str > , Error > { self . raw_name () . map (| x | demangle_auto (x , self . language)) } }
};
}
