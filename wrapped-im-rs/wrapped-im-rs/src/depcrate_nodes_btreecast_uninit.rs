// Generated macro for cast_uninit (function)
macro_rules! Depcrate_nodes_btreecast_uninit {
() => {
// Module: crate::nodes::btree
// Provides: {"cast_uninit"}
// Dependencies: {}
# [cfg (feature = "pool")] # [allow (unsafe_code)] unsafe fn cast_uninit < A > (target : & mut A) -> & mut mem :: MaybeUninit < A > { & mut * (target as * mut A as * mut mem :: MaybeUninit < A >) }
};
}
