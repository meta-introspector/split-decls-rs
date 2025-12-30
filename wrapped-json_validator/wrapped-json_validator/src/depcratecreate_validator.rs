// Generated macro for create_validator (function)
macro_rules! Depcratecreate_validator {
() => {
// Module: crate
// Provides: {"create_validator"}
// Dependencies: {}
unsafe fn create_validator (schema : * const u8 , schema_len : usize , handle : * mut usize) -> Result < () > { unsafe { let schema = json_from_raw_parts (schema , schema_len) ? ; let compiled = Validator :: new (& schema) . map_err (| error | Error :: new (E_INVALIDARG , error . to_string ())) ? ; if handle . is_null () { return Err (E_POINTER . into ()) ; } * handle = Box :: into_raw (Box :: new (compiled)) as usize ; Ok (()) } }
};
}
