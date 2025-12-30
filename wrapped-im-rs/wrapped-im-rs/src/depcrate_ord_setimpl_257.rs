// Generated macro for impl_257 (impl)
macro_rules! Depcrate_ord_setimpl_257 {
() => {
// Module: crate::ord::set
// Provides: {"impl_257"}
// Dependencies: {}
impl < A : Ord + Hash > Hash for OrdSet < A > { fn hash < H > (& self , state : & mut H) where H : Hasher , { for i in self . iter () { i . hash (state) ; } } }
};
}
