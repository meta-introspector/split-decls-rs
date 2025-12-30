// Generated macro for CreateJsonValidator (function)
macro_rules! DepcrateCreateJsonValidator {
() => {
// Module: crate
// Provides: {"CreateJsonValidator"}
// Dependencies: {}
# [no_mangle] unsafe extern "system" fn CreateJsonValidator (schema : * const u8 , schema_len : usize , handle : * mut usize ,) -> HRESULT { unsafe { create_validator (schema , schema_len , handle) . into () } }
};
}
