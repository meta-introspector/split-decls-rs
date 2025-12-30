// Generated macro for impl_197 (impl)
macro_rules! Depcrate_unitimpl_197 {
() => {
// Module: crate::unit
// Provides: {"impl_197"}
// Dependencies: {}
impl std :: hash :: Hash for Kind { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { match self { Kind :: Label (s) => { 0 . hash (state) ; s . dyn_hash (state) } Kind :: Dynamic (label) => { 1 . hash (state) ; label . dyn_hash (state) ; } } } }
};
}
