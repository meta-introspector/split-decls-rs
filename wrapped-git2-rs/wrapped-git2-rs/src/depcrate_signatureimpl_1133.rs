// Generated macro for impl_1133 (impl)
macro_rules! Depcrate_signatureimpl_1133 {
() => {
// Module: crate::signature
// Provides: {"impl_1133"}
// Dependencies: {}
impl PartialEq for Signature < '_ > { fn eq (& self , other : & Self) -> bool { self . when () == other . when () && self . email_bytes () == other . email_bytes () && self . name_bytes () == other . name_bytes () } }
};
}
