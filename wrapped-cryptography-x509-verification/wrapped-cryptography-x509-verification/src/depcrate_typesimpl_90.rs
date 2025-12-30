// Generated macro for impl_90 (impl)
macro_rules! Depcrate_typesimpl_90 {
() => {
// Module: crate::types
// Provides: {"impl_90"}
// Dependencies: {}
impl < 'a > DNSName < 'a > { pub fn new (value : & 'a str) -> Option < Self > { if value . is_empty () || value . len () > 253 { None } else { for label in value . split ('.') { if label . is_empty () || label . len () > 63 || label . starts_with ('-') || label . ends_with ('-') { return None ; } if ! label . chars () . all (| c | c . is_ascii_alphanumeric () || c == '-') { return None ; } } asn1 :: IA5String :: new (value) . map (Self) } } pub fn as_str (& self) -> & 'a str { self . 0 . as_str () } # [doc = " Return this `DNSName`'s parent domain, if it has one."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use cryptography_x509_verification::types::DNSName;"] # [doc = " let domain = DNSName::new(\"foo.example.com\").unwrap();"] # [doc = " assert_eq!(domain.parent().unwrap().as_str(), \"example.com\");"] # [doc = " ```"] pub fn parent (& self) -> Option < Self > { match self . as_str () . split_once ('.') { Some ((_ , parent)) => Self :: new (parent) , None => None , } } # [doc = " Returns this DNS name's labels, in reversed order"] # [doc = " (from top-level domain to most-specific subdomain)."] fn rlabels (& self) -> impl Iterator < Item = & '_ str > { self . as_str () . rsplit ('.') } # [doc = " Returns true if this domain is a subdomain of the other domain."] fn is_subdomain_of (& self , other : & DNSName < '_ >) -> bool { self . as_str () . len () > other . as_str () . len () && self . rlabels () . zip (other . rlabels ()) . all (| (a , o) | a . eq_ignore_ascii_case (o)) } }
};
}
