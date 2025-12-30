// Generated macro for impl_112 (impl)
macro_rules! Depcrate_mapref_oneimpl_112 {
() => {
// Module: crate::mapref::one
// Provides: {"impl_112"}
// Dependencies: {}
impl < 'a , K : Eq + Hash + Debug , T : Debug + ? Sized > Debug for MappedRefMut < 'a , K , T > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("MappedRefMut") . field ("k" , & self . k) . field ("v" , & self . v) . finish () } }
};
}
