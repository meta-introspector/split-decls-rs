// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
impl elliptic_curve :: Curve for NistP384 { # [doc = " 48-byte serialized field elements."] type FieldBytesSize = U48 ; # [doc = " 384-bit integer type used for internally representing field elements."] type Uint = U384 ; # [doc = " Order of NIST P-384's elliptic curve group (i.e. scalar modulus)."] const ORDER : Odd < U384 > = Odd :: < U384 > :: from_be_hex (ORDER_HEX) ; }
};
}
