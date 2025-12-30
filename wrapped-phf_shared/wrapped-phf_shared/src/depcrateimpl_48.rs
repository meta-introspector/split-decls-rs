// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
# [cfg (feature = "unicase")] impl < S > PhfHash for unicase :: Ascii < S > where unicase :: Ascii < S > : Hash , { # [inline] fn phf_hash < H : Hasher > (& self , state : & mut H) { self . hash (state) } }
};
}
