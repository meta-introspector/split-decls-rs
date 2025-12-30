// Generated macro for impl_30 (impl)
macro_rules! Depcrate_snapshot_signatureimpl_30 {
() => {
// Module: crate::snapshot::signature
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a > ResolvedSignature < 'a > { pub (crate) fn try_new (new_email : Option < & 'a BString > , matched_email : & 'a BStr , current_email : & '_ BStr , new_name : Option < & 'a BString > ,) -> Option < Self > { let new_email = new_email . map (| n | n . as_bstr ()) . or_else (| | (matched_email != current_email) . then_some (matched_email)) ; match (new_email , new_name) { (None , None) => None , (new_email , new_name) => Some (ResolvedSignature { email : new_email . map (| v | v . as_bstr ()) , name : new_name . map (| v | v . as_bstr ()) , }) , } } }
};
}
