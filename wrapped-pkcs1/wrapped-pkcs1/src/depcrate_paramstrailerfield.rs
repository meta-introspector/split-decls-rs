// Generated macro for TrailerField (enum)
macro_rules! Depcrate_paramsTrailerField {
() => {
// Module: crate::params
// Provides: {"TrailerField"}
// Dependencies: {}
# [doc = " `TrailerField` as defined in [RFC 8017 Appendix 2.3]."] # [doc = " ```text"] # [doc = " TrailerField ::= INTEGER { trailerFieldBC(1) }"] # [doc = " ```"] # [doc = " [RFC 8017 Appendix 2.3]: https://datatracker.ietf.org/doc/html/rfc8017#appendix-A.2.3"] # [derive (Clone , Debug , Copy , PartialEq , Eq)] # [repr (u8)] # [derive (Default)] pub enum TrailerField { # [doc = " the only supported value (0xbc, default)"] # [default] BC = 1 , }
};
}
