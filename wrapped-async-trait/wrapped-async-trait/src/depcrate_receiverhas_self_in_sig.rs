// Generated macro for has_self_in_sig (function)
macro_rules! Depcrate_receiverhas_self_in_sig {
() => {
// Module: crate::receiver
// Provides: {"has_self_in_sig"}
// Dependencies: {}
pub fn has_self_in_sig (sig : & mut Signature) -> bool { let mut visitor = HasSelf (false) ; visitor . visit_signature_mut (sig) ; visitor . 0 }
};
}
