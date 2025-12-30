// Generated macro for ValidateJson (function)
macro_rules! DepcrateValidateJson {
() => {
// Module: crate
// Provides: {"ValidateJson"}
// Dependencies: {}
# [no_mangle] unsafe extern "system" fn ValidateJson (handle : usize , value : * const u8 , value_len : usize , sanitized_value : * mut * mut u8 , sanitized_value_len : * mut usize ,) -> HRESULT { unsafe { validate (handle , value , value_len , sanitized_value , sanitized_value_len ,) . into () } }
};
}
