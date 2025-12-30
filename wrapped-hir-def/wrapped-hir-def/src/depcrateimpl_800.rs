// Generated macro for impl_800 (impl)
macro_rules! Depcrateimpl_800 {
() => {
// Module: crate
// Provides: {"impl_800"}
// Dependencies: {}
impl < N : AstIdNode > Hash for ItemLoc < N > { fn hash < H : Hasher > (& self , state : & mut H) { self . container . hash (state) ; self . id . hash (state) ; } }
};
}
