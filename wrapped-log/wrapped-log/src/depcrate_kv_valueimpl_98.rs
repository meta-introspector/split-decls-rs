// Generated macro for impl_98 (impl)
macro_rules! Depcrate_kv_valueimpl_98 {
() => {
// Module: crate::kv::value
// Provides: {"impl_98"}
// Dependencies: {}
# [cfg (feature = "kv_sval")] impl < 'v > sval_ref :: ValueRef < 'v > for Value < 'v > { fn stream_ref < S : sval :: Stream < 'v > + ? Sized > (& self , stream : & mut S) -> sval :: Result { sval_ref :: ValueRef :: stream_ref (& self . inner , stream) } }
};
}
