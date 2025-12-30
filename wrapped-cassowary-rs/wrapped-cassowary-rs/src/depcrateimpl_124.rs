// Generated macro for impl_124 (impl)
macro_rules! Depcrateimpl_124 {
() => {
// Module: crate
// Provides: {"impl_124"}
// Dependencies: {}
impl :: std :: hash :: Hash for Constraint { fn hash < H : :: std :: hash :: Hasher > (& self , hasher : & mut H) { use :: std :: ops :: Deref ; hasher . write_usize (self . 0 . deref () as * const _ as usize) ; } }
};
}
