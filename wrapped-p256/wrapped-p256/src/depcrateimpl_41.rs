// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl elliptic_curve :: Curve for NistP256 { # [doc = " 32-byte serialized field elements."] type FieldBytesSize = U32 ; # [doc = " 256-bit integer type used for internally representing field elements."] type Uint = U256 ; # [doc = " Order of NIST P-256's elliptic curve group (i.e. scalar modulus)."] const ORDER : Odd < U256 > = Odd :: < U256 > :: from_be_hex (ORDER_HEX) ; }
};
}
