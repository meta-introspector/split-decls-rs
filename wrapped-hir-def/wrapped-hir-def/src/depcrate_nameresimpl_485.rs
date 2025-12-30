// Generated macro for impl_485 (impl)
macro_rules! Depcrate_nameresimpl_485 {
() => {
// Module: crate::nameres
// Provides: {"impl_485"}
// Dependencies: {}
impl std :: hash :: Hash for LocalDefMap { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { let LocalDefMap { extern_prelude } = self ; extern_prelude . len () . hash (state) ; for (name , (crate_root , extern_crate)) in extern_prelude { name . hash (state) ; crate_root . hash (state) ; extern_crate . hash (state) ; } } }
};
}
