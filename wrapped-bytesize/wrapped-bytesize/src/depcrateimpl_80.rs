// Generated macro for impl_80 (impl)
macro_rules! Depcrateimpl_80 {
() => {
// Module: crate
// Provides: {"impl_80"}
// Dependencies: {}
impl < T > ops :: Add < T > for ByteSize where T : Into < u64 > , { type Output = ByteSize ; # [inline (always)] fn add (self , rhs : T) -> ByteSize { ByteSize (self . 0 + (rhs . into ())) } }
};
}
