// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [doc = " Runs the credential interaction"] pub fn main (credential : impl Credential) { let result = doit (credential) . map_err (| e | Error :: Other (e)) ; if result . is_err () { serde_json :: to_writer (std :: io :: stdout () , & result) . expect ("failed to serialize credential provider error") ; println ! () ; } }
};
}
