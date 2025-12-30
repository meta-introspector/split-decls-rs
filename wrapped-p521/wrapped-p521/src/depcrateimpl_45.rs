// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl elliptic_curve :: Curve for NistP521 { # [doc = " 66-byte serialized field elements."] type FieldBytesSize = U66 ; # [doc = " 521-bit integer type used for internally representing field elements."] type Uint = Uint ; # [doc = " Order of NIST P-521's elliptic curve group (i.e. scalar modulus)."] const ORDER : Odd < Uint > = Odd :: < Uint > :: from_be_hex (ORDER_HEX) ; }
};
}
