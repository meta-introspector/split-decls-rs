// Generated macro for SignatureHelp (struct)
macro_rules! Depcrate_signature_helpSignatureHelp {
() => {
// Module: crate::signature_help
// Provides: {"SignatureHelp"}
// Dependencies: {}
# [doc = " Contains information about an item signature as seen from a use site."] # [doc = ""] # [doc = " This includes the \"active parameter\", which is the parameter whose value is currently being"] # [doc = " edited."] # [derive (Debug)] pub struct SignatureHelp { pub doc : Option < Documentation > , pub signature : String , pub active_parameter : Option < usize > , parameters : Vec < TextRange > , }
};
}
