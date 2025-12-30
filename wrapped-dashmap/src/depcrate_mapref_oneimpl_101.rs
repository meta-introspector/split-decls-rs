// Generated macro for impl_101 (impl)
macro_rules! Depcrate_mapref_oneimpl_101 {
() => {
// Module: crate::mapref::one
// Provides: {"impl_101"}
// Dependencies: {}
impl < 'a , K : Eq + Hash + Debug , V : Debug > Debug for RefMut < 'a , K , V > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("RefMut") . field ("k" , & self . k) . field ("v" , & self . v) . finish () } }
};
}
