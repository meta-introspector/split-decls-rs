// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl Hash for ArcNode { fn hash < H : Hasher > (& self , state : & mut H) { Arc :: as_ptr (& self . 0) . hash (state) ; } }
};
}
