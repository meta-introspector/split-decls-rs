// Generated macro for impl_86 (impl)
macro_rules! Depcrateimpl_86 {
() => {
// Module: crate
// Provides: {"impl_86"}
// Dependencies: {}
impl < T > ops :: Mul < T > for ByteSize where T : Into < u64 > , { type Output = ByteSize ; # [inline (always)] fn mul (self , rhs : T) -> ByteSize { ByteSize (self . 0 * rhs . into ()) } }
};
}
