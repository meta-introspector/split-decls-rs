// Generated macro for impl_104 (impl)
macro_rules! Depcrate_typesimpl_104 {
() => {
// Module: crate::types
// Provides: {"impl_104"}
// Dependencies: {}
impl < 'a > RFC822Constraint < 'a > { pub fn new (constraint : & 'a str) -> Option < Self > { if let Some (constraint) = constraint . strip_prefix ('.') { Some (Self :: InDomain (DNSName :: new (constraint) ?)) } else if let Some (email) = RFC822Name :: new (constraint) { Some (Self :: Exact (email)) } else { Some (Self :: OnDomain (DNSName :: new (constraint) ?)) } } pub fn matches (& self , email : & RFC822Name < '_ >) -> bool { match self { Self :: Exact (pat) => pat == email , Self :: OnDomain (pat) => & email . domain == pat , Self :: InDomain (pat) => email . domain . is_subdomain_of (pat) , } } }
};
}
