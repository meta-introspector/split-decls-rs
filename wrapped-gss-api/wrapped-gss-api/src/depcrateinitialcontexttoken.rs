// Generated macro for InitialContextToken (struct)
macro_rules! DepcrateInitialContextToken {
() => {
// Module: crate
// Provides: {"InitialContextToken"}
// Dependencies: {}
# [doc = " InitialContextToken as defined in [RFC 1508 Appendix B]."] # [doc = ""] # [doc = " ```text"] # [doc = " InitialContextToken ::="] # [doc = " -- option indication (delegation, etc.) indicated within"] # [doc = " -- mechanism-specific token"] # [doc = " [APPLICATION 0] IMPLICIT SEQUENCE {"] # [doc = "     thisMech MechType,"] # [doc = "     innerContextToken ANY DEFINED BY thisMec"] # [doc = "          -- contents mechanism-specific"] # [doc = "     }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 1508 Appendix B]: https://datatracker.ietf.org/doc/html/rfc1508#appendix-B"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct InitialContextToken < 'a > { # [doc = " mechanism type OID"] pub this_mech : MechType , # [doc = " mechanism-specific content"] pub inner_context_token : AnyRef < 'a > , }
};
}
