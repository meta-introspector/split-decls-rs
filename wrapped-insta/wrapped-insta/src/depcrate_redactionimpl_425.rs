// Generated macro for impl_425 (impl)
macro_rules! Depcrate_redactionimpl_425 {
() => {
// Module: crate::redaction
// Provides: {"impl_425"}
// Dependencies: {}
impl Redaction { # [doc = " Performs the redaction of the value at the given path."] fn redact (& self , value : Content , path : & [PathItem]) -> Content { match * self { Redaction :: Static (ref new_val) => new_val . clone () , Redaction :: Dynamic (ref callback) => callback (value , ContentPath (path)) , } } }
};
}
