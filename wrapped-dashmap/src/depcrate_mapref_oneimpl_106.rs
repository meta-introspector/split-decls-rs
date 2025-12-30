// Generated macro for impl_106 (impl)
macro_rules! Depcrate_mapref_oneimpl_106 {
() => {
// Module: crate::mapref::one
// Provides: {"impl_106"}
// Dependencies: {}
impl < 'a , K : Eq + Hash + Debug , T : Debug + ? Sized > Debug for MappedRef < 'a , K , T > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("MappedRef") . field ("k" , & self . k) . field ("v" , & self . v) . finish () } }
};
}
