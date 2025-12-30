// Generated macro for impl_1627 (impl)
macro_rules! Depcrateimpl_1627 {
() => {
// Module: crate
// Provides: {"impl_1627"}
// Dependencies: {}
impl std :: hash :: Hash for Compiler { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . stage . hash (state) ; self . host . hash (state) ; } }
};
}
