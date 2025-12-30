// Generated macro for impl_175 (impl)
macro_rules! Depcrate_idimpl_175 {
() => {
// Module: crate::id
// Provides: {"impl_175"}
// Dependencies: {}
impl < N : ToOptionString + hash :: Hash > hash :: Hash for ItemIdentifier < N > { fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . name . hash (state) ; } }
};
}
