// Generated macro for impl_23 (impl)
macro_rules! Depcrate_as_symbol_nameimpl_23 {
() => {
// Module: crate::as_symbol_name
// Provides: {"impl_23"}
// Dependencies: {}
impl Sealed for & str { fn symbol_name < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { self . as_bytes () . symbol_name (function) } }
};
}
