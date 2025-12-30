// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl elliptic_curve :: Curve for NistP224 { # [doc = " 28-byte serialized field elements."] type FieldBytesSize = U28 ; # [doc = " Big integer type used for representing field elements."] type Uint = Uint ; # [doc = " Order of NIST P-224's elliptic curve group (i.e. scalar modulus)."] const ORDER : Odd < Uint > = Odd :: < Uint > :: from_be_hex (ORDER_HEX) ; }
};
}
