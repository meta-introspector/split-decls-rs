// Generated macro for impl_37 (impl)
macro_rules! Depcrate_as_symbol_nameimpl_37 {
() => {
// Module: crate::as_symbol_name
// Provides: {"impl_37"}
// Dependencies: {}
impl < const N : usize > Sealed for [u8 ; N] { fn symbol_name < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , Error > ,) -> Result < R , Error > { self . as_slice () . symbol_name (function) } }
};
}
