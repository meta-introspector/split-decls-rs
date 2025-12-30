// Generated macro for impl_33 (impl)
macro_rules! Depcrate_as_symbol_nameimpl_33 {
() => {
// Module: crate::as_symbol_name
// Provides: {"impl_33"}
// Dependencies: {}
impl Sealed for CString { fn symbol_name < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { function (self . as_ptr ()) } }
};
}
