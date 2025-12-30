// Generated macro for impl_68 (impl)
macro_rules! Depcrateimpl_68 {
() => {
// Module: crate
// Provides: {"impl_68"}
// Dependencies: {}
impl PhfHash for char { # [inline] fn phf_hash < H : Hasher > (& self , state : & mut H) { (* self as u32) . phf_hash (state) } }
};
}
