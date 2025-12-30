// Generated macro for impl_93 (impl)
macro_rules! Depcrate_typesimpl_93 {
() => {
// Module: crate::types
// Provides: {"impl_93"}
// Dependencies: {}
impl < 'a > DNSPattern < 'a > { pub fn new (pat : & 'a str) -> Option < Self > { if let Some (pat) = pat . strip_prefix ("*.") { DNSName :: new (pat) . map (Self :: Wildcard) } else { DNSName :: new (pat) . map (Self :: Exact) } } pub fn matches (& self , name : & DNSName < '_ >) -> bool { match self { Self :: Exact (pat) => pat == name , Self :: Wildcard (pat) => match name . parent () { Some (ref parent) => pat == parent , None => false , } , } } # [doc = " Returns the inner `DNSName` within this `DNSPattern`, e.g."] # [doc = " `foo.com` for `*.foo.com` or `example.com` for `example.com`."] # [doc = ""] # [doc = " This API must not be used to bypass pattern matching; it exists"] # [doc = " solely to enable checks that only require the inner name, such"] # [doc = " as Name Constraint checks."] pub fn inner_name (& self) -> & DNSName < 'a > { match self { DNSPattern :: Exact (dnsname) => dnsname , DNSPattern :: Wildcard (dnsname) => dnsname , } } }
};
}
