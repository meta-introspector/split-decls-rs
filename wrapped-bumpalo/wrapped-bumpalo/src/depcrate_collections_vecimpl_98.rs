// Generated macro for impl_98 (impl)
macro_rules! Depcrate_collections_vecimpl_98 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_98"}
// Dependencies: {}
impl < 'bump , T : 'bump + Hash > Hash for Vec < 'bump , T > { # [inline] fn hash < H : hash :: Hasher > (& self , state : & mut H) { Hash :: hash (& * * self , state) } }
};
}
