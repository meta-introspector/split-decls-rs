// Generated macro for impl_18 (impl)
macro_rules! Depcrate_hashimpl_18 {
() => {
// Module: crate::hash
// Provides: {"impl_18"}
// Dependencies: {}
impl NumHash for i128 { # [inline] fn num_hash < H : Hasher > (& self , state : & mut H) { const MINP1 : i128 = i128 :: MIN + 1 ; match * self { i128 :: MAX | MINP1 => 0i128 . hash (state) , i128 :: MIN => (- 1i128) . hash (state) , u => u . hash (state) , } } }
};
}
