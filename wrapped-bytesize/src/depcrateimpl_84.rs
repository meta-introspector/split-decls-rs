// Generated macro for impl_84 (impl)
macro_rules! Depcrateimpl_84 {
() => {
// Module: crate
// Provides: {"impl_84"}
// Dependencies: {}
impl < T > ops :: Sub < T > for ByteSize where T : Into < u64 > , { type Output = ByteSize ; # [inline (always)] fn sub (self , rhs : T) -> ByteSize { ByteSize (self . 0 - (rhs . into ())) } }
};
}
