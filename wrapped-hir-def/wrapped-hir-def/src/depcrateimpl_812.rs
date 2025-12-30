// Generated macro for impl_812 (impl)
macro_rules! Depcrateimpl_812 {
() => {
// Module: crate
// Provides: {"impl_812"}
// Dependencies: {}
impl < N : AstIdNode > Hash for AssocItemLoc < N > { fn hash < H : Hasher > (& self , state : & mut H) { self . container . hash (state) ; self . id . hash (state) ; } }
};
}
