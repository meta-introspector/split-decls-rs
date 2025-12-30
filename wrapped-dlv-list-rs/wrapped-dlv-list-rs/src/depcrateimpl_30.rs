// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl < T > Hash for VecList < T > where T : Hash , { fn hash < StateHasher > (& self , state : & mut StateHasher) where StateHasher : Hasher , { self . len () . hash (state) ; for value in self { value . hash (state) ; } } }
};
}
