// Generated macro for from_str (function)
macro_rules! Depcrate_common_strict_transport_securityfrom_str {
() => {
// Module: crate::common::strict_transport_security
// Provides: {"from_str"}
// Dependencies: {}
fn from_str (s : & str) -> Result < StrictTransportSecurity , Error > { s . split (';') . map (str :: trim) . map (| sub | { if sub . eq_ignore_ascii_case ("includeSubdomains") { Some (Directive :: IncludeSubdomains) } else { let mut sub = sub . splitn (2 , '=') ; match (sub . next () , sub . next ()) { (Some (left) , Some (right)) if left . trim () . eq_ignore_ascii_case ("max-age") => { right . trim () . trim_matches ('"') . parse () . ok () . map (Directive :: MaxAge) } _ => Some (Directive :: Unknown) , } } }) . try_fold ((None , None) , | res , dir | match (res , dir) { ((None , sub) , Some (Directive :: MaxAge (age))) => Some ((Some (age) , sub)) , ((age , None) , Some (Directive :: IncludeSubdomains)) => Some ((age , Some (()))) , ((Some (_) , _) , Some (Directive :: MaxAge (_))) | ((_ , Some (_)) , Some (Directive :: IncludeSubdomains)) | (_ , None) => None , (res , _) => Some (res) , }) . and_then (| res | match res { (Some (age) , sub) => Some (StrictTransportSecurity { max_age : Duration :: from_secs (age) . into () , include_subdomains : sub . is_some () , }) , _ => None , }) . ok_or_else (Error :: invalid) }
};
}
