// Generated macro for impl_7 (impl)
macro_rules! Depcrate_errorimpl_7 {
() => {
// Module: crate::error
// Provides: {"impl_7"}
// Dependencies: {}
impl UninitializedFieldError { # [doc = " Create a new `UnitializedFieldError` for the specified field name."] pub fn new (field_name : & 'static str) -> Self { UninitializedFieldError (field_name) } # [doc = " Get the name of the first-declared field that wasn't initialized"] pub fn field_name (& self) -> & 'static str { self . 0 } }
};
}
