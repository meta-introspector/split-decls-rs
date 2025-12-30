// Generated macro for impl_50 (impl)
macro_rules! Depcrate_features_impl_allocimpl_50 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_50"}
// Dependencies: {}
impl VecWriter { # [doc = " Create a new vec writer with the given capacity"] pub fn with_capacity (cap : usize) -> Self { Self { inner : Vec :: with_capacity (cap) , } } # [allow (dead_code)] pub (crate) fn collect (self) -> Vec < u8 > { self . inner } }
};
}
