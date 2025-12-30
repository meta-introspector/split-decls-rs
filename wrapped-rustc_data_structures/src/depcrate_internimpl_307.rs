// Generated macro for impl_307 (impl)
macro_rules! Depcrate_internimpl_307 {
() => {
// Module: crate::intern
// Provides: {"impl_307"}
// Dependencies: {}
impl < 'a , T > Hash for Interned < 'a , T > where T : Hash , { # [inline] fn hash < H : Hasher > (& self , s : & mut H) { ptr :: hash (self . 0 , s) } }
};
}
