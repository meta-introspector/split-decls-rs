// Generated macro for impl_97 (impl)
macro_rules! Depcrate_mapref_oneimpl_97 {
() => {
// Module: crate::mapref::one
// Provides: {"impl_97"}
// Dependencies: {}
impl < 'a , K : Eq + Hash + Debug , V : Debug > Debug for Ref < 'a , K , V > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Ref") . field ("k" , & self . k) . field ("v" , & self . v) . finish () } }
};
}
