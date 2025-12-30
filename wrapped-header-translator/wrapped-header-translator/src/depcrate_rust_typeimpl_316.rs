// Generated macro for impl_316 (impl)
macro_rules! Depcrate_rust_typeimpl_316 {
() => {
// Module: crate::rust_type
// Provides: {"impl_316"}
// Dependencies: {}
impl Drop for AttributeParser < '_ , '_ > { fn drop (& mut self) { if ! std :: thread :: panicking () && self . name != self . expected_name { error ! (? self , "could not extract all attributes") ; } } }
};
}
