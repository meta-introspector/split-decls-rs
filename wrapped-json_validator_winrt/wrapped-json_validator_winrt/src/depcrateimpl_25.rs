// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl bindings :: IJsonValidator_Impl for JsonValidator_Impl { fn Validate (& self , value : & HSTRING) -> Result < HSTRING > { let value = json_from_hstring (value) ? ; if self . schema . is_valid (& value) { Ok (value . to_string () . into ()) } else { let message = self . schema . validate (& value) . unwrap_err () . next () . map_or (String :: new () , | error | error . to_string ()) ; Err (Error :: new (E_INVALIDARG , message)) } } }
};
}
