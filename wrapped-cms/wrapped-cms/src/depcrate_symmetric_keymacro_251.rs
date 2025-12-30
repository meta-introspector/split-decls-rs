// Generated macro for macro_251 (macro)
macro_rules! Depcrate_symmetric_keymacro_251 {
() => {
// Module: crate::symmetric_key
// Provides: {"macro_251"}
// Dependencies: {}
impl_str_enum ! (# [doc = " The `PSKCKeyUsage` type is defined in [RFC 6031 Section 3.3.4]."] # [doc = ""] # [doc = " ```text"] # [doc = "    PSKCKeyUsage ::= UTF8String (\"OTP\" | \"CR\" | \"Encrypt\" |"] # [doc = "                     \"Integrity\" | \"Verify\" | \"Unlock\" | \"Decrypt\" |"] # [doc = "                     \"KeyWrap\" | \"Unwrap\" | \"Derive\" | \"Generate\")"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6031 Section 3.3.4]: https://datatracker.ietf.org/doc/html/rfc6031#section-3.3.4"] # [derive (Copy , Clone , PartialEq , Eq)] pub enum PSKCKeyUsage { # [doc = " \"OTP\""] Otp => "Otp" , # [doc = " \"CR\""] Cr => "CR" , # [doc = " \"Encrypt\""] Encrypt => "Encrypt" , # [doc = " \"Integrity\""] Integrity => "Integrity" , # [doc = " \"Verify\""] Verify => "Verify" , # [doc = " \"Unlock\""] Unlock => "Unlock" , # [doc = " \"Decrypt\""] Decrypt => "Decrypt" , # [doc = " \"KeyWrap\""] KeyWrap => "KeyWrap" , # [doc = " \"Unwrap\""] Unwrap => "Unwrap" , # [doc = " \"Derive\""] Derive => "Derive" , # [doc = " \"Generate\""] Generate => "Generate" , }) ;
};
}
