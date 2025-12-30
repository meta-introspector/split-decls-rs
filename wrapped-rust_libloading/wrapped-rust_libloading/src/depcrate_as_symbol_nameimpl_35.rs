// Generated macro for impl_35 (impl)
macro_rules! Depcrate_as_symbol_nameimpl_35 {
() => {
// Module: crate::as_symbol_name
// Provides: {"impl_35"}
// Dependencies: {}
impl Sealed for & [u8] { fn symbol_name < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { if crate :: util :: check_null_bytes (self) ? { function (self . as_ptr () . cast ()) } else { let copy = crate :: util :: copy_and_push (self , 0) ; function (copy . as_ptr () . cast ()) } } }
};
}
