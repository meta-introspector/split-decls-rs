// Generated macro for DNSPattern (enum)
macro_rules! Depcrate_typesDNSPattern {
() => {
// Module: crate::types
// Provides: {"DNSPattern"}
// Dependencies: {}
# [doc = " Represents either a DNS name or a DNS wildcard for use in X.509 name"] # [doc = " matching."] # [doc = ""] # [doc = " A `DNSPattern` represents a subset of the domain name wildcard matching"] # [doc = " behavior defined in [RFC 6125 6.4.3]. In particular, all DNS patterns"] # [doc = " must either be exact matches (post-normalization) *or* a single wildcard"] # [doc = " matching a full label in the left-most label position. Partial label matching"] # [doc = " (e.g. `f*o.example.com`) is not supported, nor is non-left-most matching"] # [doc = " (e.g. `foo.*.example.com`)."] # [doc = ""] # [doc = " [RFC 6125 6.4.3]: https://datatracker.ietf.org/doc/html/rfc6125#section-6.4.3"] # [derive (Debug , PartialEq)] pub enum DNSPattern < 'a > { Exact (DNSName < 'a >) , Wildcard (DNSName < 'a >) , }
};
}
