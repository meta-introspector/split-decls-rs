// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl PhfHash for Entry { fn phf_hash < H > (& self , state : & mut H) where H : Hasher , { self . key . phf_hash (state) } }
};
}
