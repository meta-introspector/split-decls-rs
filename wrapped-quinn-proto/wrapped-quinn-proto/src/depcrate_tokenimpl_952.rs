// Generated macro for impl_952 (impl)
macro_rules! Depcrate_tokenimpl_952 {
() => {
// Module: crate::token
// Provides: {"impl_952"}
// Dependencies: {}
impl ResetToken { pub (crate) fn new (key : & dyn HmacKey , id : ConnectionId) -> Self { let mut signature = vec ! [0 ; key . signature_len ()] ; key . sign (& id , & mut signature) ; let mut result = [0 ; RESET_TOKEN_SIZE] ; result . copy_from_slice (& signature [.. RESET_TOKEN_SIZE]) ; result . into () } }
};
}
