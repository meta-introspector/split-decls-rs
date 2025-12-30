// Generated macro for Version (enum)
macro_rules! Depcrate_versionVersion {
() => {
// Module: crate::version
// Provides: {"Version"}
// Dependencies: {}
# [doc = " Version identifier for PKCS#1 documents as defined in"] # [doc = " [RFC 8017 Appendix 1.2]."] # [doc = ""] # [doc = " > version is the version number, for compatibility with future"] # [doc = " > revisions of this document.  It SHALL be 0 for this version of the"] # [doc = " > document, unless multi-prime is used; in which case, it SHALL be 1."] # [doc = ""] # [doc = " ```text"] # [doc = " Version ::= INTEGER { two-prime(0), multi(1) }"] # [doc = "    (CONSTRAINED BY"] # [doc = "    {-- version must be multi if otherPrimeInfos present --})"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 8017 Appendix 1.2]: https://datatracker.ietf.org/doc/html/rfc8017#appendix-A.1.2"] # [derive (Copy , Clone , Debug , Eq , PartialEq , PartialOrd , Ord)] # [repr (u8)] pub enum Version { # [doc = " Denotes a `two-prime` key"] TwoPrime = 0 , # [doc = " Denotes a `multi` (i.e. multi-prime) key"] Multi = 1 , }
};
}
