// Generated macro for impl_97 (impl)
macro_rules! Depcrate_kv_valueimpl_97 {
() => {
// Module: crate::kv::value
// Provides: {"impl_97"}
// Dependencies: {}
# [cfg (feature = "kv_sval")] impl < 'v > sval :: Value for Value < 'v > { fn stream < 'sval , S : sval :: Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S) -> sval :: Result { sval :: Value :: stream (& self . inner , stream) } }
};
}
