// Generated macro for impl_25 (impl)
macro_rules! Depcrate_as_symbol_nameimpl_25 {
() => {
// Module: crate::as_symbol_name
// Provides: {"impl_25"}
// Dependencies: {}
impl Sealed for & String { fn symbol_name < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { self . as_str () . symbol_name (function) } }
};
}
