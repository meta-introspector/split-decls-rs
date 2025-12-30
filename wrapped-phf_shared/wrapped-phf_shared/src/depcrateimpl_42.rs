// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl PhfHash for str { # [inline] fn phf_hash < H : Hasher > (& self , state : & mut H) { self . as_bytes () . phf_hash (state) } }
};
}
