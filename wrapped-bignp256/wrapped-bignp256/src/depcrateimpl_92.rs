// Generated macro for impl_92 (impl)
macro_rules! Depcrateimpl_92 {
() => {
// Module: crate
// Provides: {"impl_92"}
// Dependencies: {}
impl elliptic_curve :: Curve for BignP256 { # [doc = " 256-bit integer type used for internally representing field elements."] type FieldBytesSize = U32 ; type Uint = U256 ; # [doc = " Order of BIGN P-256's elliptic curve group (i.e. scalar modulus)."] const ORDER : Odd < U256 > = Odd :: < U256 > :: from_be_hex (ORDER_HEX) ; }
};
}
