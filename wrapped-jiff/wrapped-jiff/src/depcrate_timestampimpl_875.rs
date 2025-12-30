// Generated macro for impl_875 (impl)
macro_rules! Depcrate_timestampimpl_875 {
() => {
// Module: crate::timestamp
// Provides: {"impl_875"}
// Dependencies: {}
impl core :: hash :: Hash for Timestamp { # [inline] fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { self . as_second_ranged () . get () . hash (state) ; self . subsec_nanosecond_ranged () . get () . hash (state) ; } }
};
}
