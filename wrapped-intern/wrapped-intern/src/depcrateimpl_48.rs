// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
impl < T : Internable + ? Sized > Hash for Interned < T > { fn hash < H : Hasher > (& self , state : & mut H) { state . write_usize (Arc :: as_ptr (& self . arc) as * const () as usize) } }
};
}
