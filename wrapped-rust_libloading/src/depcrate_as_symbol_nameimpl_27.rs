// Generated macro for impl_27 (impl)
macro_rules! Depcrate_as_symbol_nameimpl_27 {
() => {
// Module: crate::as_symbol_name
// Provides: {"impl_27"}
// Dependencies: {}
impl Sealed for String { fn symbol_name < R > (mut self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { if crate :: util :: check_null_bytes (self . as_bytes ()) ? { function (self . as_ptr () . cast ()) } else { self . push ('\0') ; function (self . as_ptr () . cast ()) } } }
};
}
