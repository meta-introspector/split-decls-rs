// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
# [allow (unused_must_use)] impl < T , const CAP : usize > Extend < T > for ArrayDeque < T , CAP , Saturating > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = T > , { self . extend_back (iter) ; } }
};
}
