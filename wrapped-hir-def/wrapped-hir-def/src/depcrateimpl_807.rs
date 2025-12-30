// Generated macro for impl_807 (impl)
macro_rules! Depcrateimpl_807 {
() => {
// Module: crate
// Provides: {"impl_807"}
// Dependencies: {}
impl < N : AstIdNode > Hash for AssocItemLoc < N > { fn hash < H : Hasher > (& self , state : & mut H) { self . container . hash (state) ; self . id . hash (state) ; } }
};
}
