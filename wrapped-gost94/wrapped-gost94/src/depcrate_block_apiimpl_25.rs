// Generated macro for impl_25 (impl)
macro_rules! Depcrate_block_apiimpl_25 {
() => {
// Module: crate::block_api
// Provides: {"impl_25"}
// Dependencies: {}
impl < P : Gost94Params > Clone for Gost94Core < P > { # [inline] fn clone (& self) -> Self { Self { h : self . h , n : self . n , sigma : self . sigma , _m : PhantomData , } } }
};
}
