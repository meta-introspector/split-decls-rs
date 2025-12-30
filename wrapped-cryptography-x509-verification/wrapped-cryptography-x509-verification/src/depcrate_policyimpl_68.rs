// Generated macro for impl_68 (impl)
macro_rules! Depcrate_policyimpl_68 {
() => {
// Module: crate::policy
// Provides: {"impl_68"}
// Dependencies: {}
impl Subject < '_ > { fn subject_alt_name_matches (& self , general_name : & GeneralName < '_ >) -> bool { match (general_name , self) { (GeneralName :: DNSName (pattern) , Self :: DNS (name)) => { DNSPattern :: new (pattern . 0) . is_some_and (| p | p . matches (name)) } (GeneralName :: IPAddress (addr) , Self :: IP (name)) => { IPAddress :: from_bytes (addr) == Some (* name) } _ => false , } } # [doc = " Returns true if any of the names in the given `SubjectAlternativeName`"] # [doc = " match this `Subject`."] pub fn matches (& self , san : & SubjectAlternativeName < '_ >) -> bool { san . clone () . any (| gn | self . subject_alt_name_matches (& gn)) } }
};
}
