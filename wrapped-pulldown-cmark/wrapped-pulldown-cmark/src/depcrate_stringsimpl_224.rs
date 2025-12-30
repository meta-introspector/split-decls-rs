// Generated macro for impl_224 (impl)
macro_rules! Depcrate_stringsimpl_224 {
() => {
// Module: crate::strings
// Provides: {"impl_224"}
// Dependencies: {}
impl < 'a > Hash for CowStr < 'a > { fn hash < H : Hasher > (& self , state : & mut H) { self . deref () . hash (state) ; } }
};
}
