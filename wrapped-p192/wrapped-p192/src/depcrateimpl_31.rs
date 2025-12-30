// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl elliptic_curve :: Curve for NistP192 { # [doc = " 24-byte serialized field elements."] type FieldBytesSize = U24 ; # [doc = " Big integer type used for representing field elements."] type Uint = U192 ; # [doc = " Order of NIST P-192's elliptic curve group (i.e. scalar modulus)."] const ORDER : Odd < U192 > = Odd :: < U192 > :: from_be_hex (ORDER_HEX) ; }
};
}
