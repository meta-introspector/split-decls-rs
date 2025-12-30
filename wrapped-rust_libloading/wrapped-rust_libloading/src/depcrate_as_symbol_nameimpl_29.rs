// Generated macro for impl_29 (impl)
macro_rules! Depcrate_as_symbol_nameimpl_29 {
() => {
// Module: crate::as_symbol_name
// Provides: {"impl_29"}
// Dependencies: {}
impl Sealed for & CStr { fn symbol_name < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { function (self . as_ptr ()) } }
};
}
