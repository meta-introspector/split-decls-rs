// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
# [allow (unused_must_use)] impl < T , const CAP : usize > Extend < T > for ArrayDeque < T , CAP , Wrapping > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = T > , { let take = self . capacity () - self . len () ; for elt in iter . into_iter () . take (take) { self . push_back (elt) ; } } }
};
}
