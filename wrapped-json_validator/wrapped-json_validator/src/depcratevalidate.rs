// Generated macro for validate (function)
macro_rules! Depcratevalidate {
() => {
// Module: crate
// Provides: {"validate"}
// Dependencies: {}
unsafe fn validate (handle : usize , value : * const u8 , value_len : usize , sanitized_value : * mut * mut u8 , sanitized_value_len : * mut usize ,) -> Result < () > { unsafe { if handle == 0 { return Err (E_HANDLE . into ()) ; } let value = json_from_raw_parts (value , value_len) ? ; let schema = & * (handle as * const Validator) ; if schema . is_valid (& value) { if ! sanitized_value . is_null () && ! sanitized_value_len . is_null () { let value = value . to_string () ; * sanitized_value = CoTaskMemAlloc (value . len ()) as _ ; if (* sanitized_value) . is_null () { return Err (E_OUTOFMEMORY . into ()) ; } (* sanitized_value) . copy_from (value . as_ptr () , value . len ()) ; * sanitized_value_len = value . len () ; } Ok (()) } else { let mut message = String :: new () ; if let Some (error) = schema . validate (& value) . unwrap_err () . next () { message = error . to_string () ; } Err (Error :: new (E_INVALIDARG , message)) } } }
};
}
