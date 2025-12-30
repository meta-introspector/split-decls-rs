// Generated macro for impl_31 (impl)
macro_rules! Depcrate_as_symbol_nameimpl_31 {
() => {
// Module: crate::as_symbol_name
// Provides: {"impl_31"}
// Dependencies: {}
impl Sealed for & CString { fn symbol_name < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { function (self . as_ptr ()) } }
};
}
