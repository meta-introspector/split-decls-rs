// Generated macro for impl_26 (impl)
macro_rules! Depcrate_block_apiimpl_26 {
() => {
// Module: crate::block_api
// Provides: {"impl_26"}
// Dependencies: {}
impl < P : Gost94Params > Default for Gost94Core < P > { # [inline] fn default () -> Self { Self { h : P :: H0 , n : Default :: default () , sigma : Default :: default () , _m : PhantomData , } } }
};
}
