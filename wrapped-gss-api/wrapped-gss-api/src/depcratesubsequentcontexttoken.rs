// Generated macro for SubsequentContextToken (type)
macro_rules! DepcrateSubsequentContextToken {
() => {
// Module: crate
// Provides: {"SubsequentContextToken"}
// Dependencies: {}
# [doc = " The `SubsequentContextToken` type is defined in [RFC 1508 Appendix B]."] # [doc = ""] # [doc = " ```text"] # [doc = " subsequentContextToken ::= innerContextToken ANY"] # [doc = " -- interpretation based on predecessor InitialContextToken"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 1508 Appendix B]: https://datatracker.ietf.org/doc/html/rfc1508#appendix-B"] pub type SubsequentContextToken < 'a > = AnyRef < 'a > ;
};
}
