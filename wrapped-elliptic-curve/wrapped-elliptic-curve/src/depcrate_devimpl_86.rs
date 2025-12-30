// Generated macro for impl_86 (impl)
macro_rules! Depcrate_devimpl_86 {
() => {
// Module: crate::dev
// Provides: {"impl_86"}
// Dependencies: {}
impl Reduce < U256 > for Scalar { fn reduce (w : & U256) -> Self { let (r , underflow) = w . borrowing_sub (& MockCurve :: ORDER , Limb :: ZERO) ; let underflow = Choice :: from ((underflow . 0 >> (Limb :: BITS - 1)) as u8) ; let reduced = U256 :: conditional_select (w , & r , ! underflow) ; Self (ScalarValue :: new (reduced) . unwrap ()) } }
};
}
