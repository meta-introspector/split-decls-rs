// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl bindings :: IJsonValidatorFactory_Impl for JsonValidatorFactory_Impl { fn CreateInstance (& self , schema : & HSTRING) -> Result < bindings :: JsonValidator > { let schema = json_from_hstring (schema) ? ; let schema = Validator :: new (& schema) . map_err (| error | Error :: new (E_INVALIDARG , error . to_string ())) ? ; Ok (JsonValidator { schema } . into ()) } }
};
}
