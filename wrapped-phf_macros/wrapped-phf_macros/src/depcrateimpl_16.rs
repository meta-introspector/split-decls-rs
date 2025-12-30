// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl PhfHash for Key { fn phf_hash < H > (& self , state : & mut H) where H : Hasher , { if let Some (first) = self . parsed . first () { first . phf_hash (state) ; } } }
};
}
