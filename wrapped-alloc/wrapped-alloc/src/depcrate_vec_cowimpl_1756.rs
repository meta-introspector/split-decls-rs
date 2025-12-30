// Generated macro for impl_1756 (impl)
macro_rules! Depcrate_vec_cowimpl_1756 {
() => {
// Module: crate::vec::cow
// Provides: {"impl_1756"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T > FromIterator < T > for Cow < 'a , [T] > where T : Clone , { # [track_caller] fn from_iter < I : IntoIterator < Item = T > > (it : I) -> Cow < 'a , [T] > { Cow :: Owned (FromIterator :: from_iter (it)) } }
};
}
