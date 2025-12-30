// Generated macro for impl_493 (impl)
macro_rules! Depcrate_setimpl_493 {
() => {
// Module: crate::set
// Provides: {"impl_493"}
// Dependencies: {}
impl < T , S , A > Default for HashSet < T , S , A > where S : Default , A : Default + Allocator , { # [doc = " Creates an empty `HashSet<T, S>` with the `Default` value for the hasher."] # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { Self { map : HashMap :: default () , } } }
};
}
