// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
# [cfg (feature = "unicase")] impl < S > PhfHash for unicase :: UniCase < S > where unicase :: UniCase < S > : Hash , { # [inline] fn phf_hash < H : Hasher > (& self , state : & mut H) { self . hash (state) } }
};
}
