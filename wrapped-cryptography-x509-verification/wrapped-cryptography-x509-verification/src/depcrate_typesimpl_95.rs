// Generated macro for impl_95 (impl)
macro_rules! Depcrate_typesimpl_95 {
() => {
// Module: crate::types
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'a > DNSConstraint < 'a > { pub fn new (pattern : & 'a str) -> Option < Self > { DNSName :: new (pattern) . map (Self) } # [doc = " Returns true if this `DNSConstraint` matches the given name."] # [doc = ""] # [doc = " Constraint matching is defined by RFC 5280: any DNS name that can"] # [doc = " be constructed by simply adding zero or more labels to the left-hand"] # [doc = " side of the name satisfies the name constraint."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use cryptography_x509_verification::types::{DNSConstraint, DNSName};"] # [doc = " let example_com = DNSName::new(\"example.com\").unwrap();"] # [doc = " let badexample_com = DNSName::new(\"badexample.com\").unwrap();"] # [doc = " let foo_example_com = DNSName::new(\"foo.example.com\").unwrap();"] # [doc = " assert!(DNSConstraint::new(example_com.as_str()).unwrap().matches(&example_com));"] # [doc = " assert!(DNSConstraint::new(example_com.as_str()).unwrap().matches(&foo_example_com));"] # [doc = " assert!(!DNSConstraint::new(example_com.as_str()).unwrap().matches(&badexample_com));"] # [doc = " ```"] pub fn matches (& self , name : & DNSName < '_ >) -> bool { name . as_str () . len () >= self . 0 . as_str () . len () && self . 0 . rlabels () . zip (name . rlabels ()) . all (| (a , o) | a . eq_ignore_ascii_case (o)) } }
};
}
