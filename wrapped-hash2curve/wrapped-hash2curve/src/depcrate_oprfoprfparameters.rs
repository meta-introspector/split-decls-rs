// Generated macro for OprfParameters (trait)
macro_rules! Depcrate_oprfOprfParameters {
() => {
// Module: crate::oprf
// Provides: {"OprfParameters"}
// Dependencies: {}
# [doc = " Elliptic curve parameters used by OPRF."] pub trait OprfParameters : GroupDigest < ExpandMsg : ExpandMsg < Self :: SecurityLevel , Hash : Default + FixedOutput < OutputSize : IsLess < U65536 , Output = True > > + Update , > , > + PrimeCurve { # [doc = " The `ID` parameter which identifies a particular elliptic curve"] # [doc = " as defined in [section 4 of RFC9497][oprf]."] # [doc = ""] # [doc = " [oprf]: https://www.rfc-editor.org/rfc/rfc9497.html#name-ciphersuites"] const ID : & 'static [u8] ; }
};
}
