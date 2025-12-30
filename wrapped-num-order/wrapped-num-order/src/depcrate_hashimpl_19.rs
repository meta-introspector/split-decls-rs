// Generated macro for impl_19 (impl)
macro_rules! Depcrate_hashimpl_19 {
() => {
// Module: crate::hash
// Provides: {"impl_19"}
// Dependencies: {}
impl NumHash for u128 { # [inline] fn num_hash < H : Hasher > (& self , state : & mut H) { match * self { u128 :: MAX => 1i128 . hash (state) , M127D => 0i128 . hash (state) , u if u >= M127U => ((u - M127U) as i128) . hash (state) , u => (u as i128) . hash (state) , } } }
};
}
