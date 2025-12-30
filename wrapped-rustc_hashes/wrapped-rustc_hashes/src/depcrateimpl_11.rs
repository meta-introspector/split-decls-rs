// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl std :: hash :: Hash for Hash128 { fn hash < H : std :: hash :: Hasher > (& self , h : & mut H) { h . write_u64 (self . truncate () . as_u64 ()) ; } }
};
}
