// Generated macro for impl_108 (impl)
macro_rules! Depcrate_mapref_oneimpl_108 {
() => {
// Module: crate::mapref::one
// Provides: {"impl_108"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , T : std :: fmt :: Display + ? Sized > std :: fmt :: Display for MappedRef < 'a , K , T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Display :: fmt (self . value () , f) } }
};
}
