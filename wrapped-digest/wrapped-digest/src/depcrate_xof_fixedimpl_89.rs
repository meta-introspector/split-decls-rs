// Generated macro for impl_89 (impl)
macro_rules! Depcrate_xof_fixedimpl_89 {
() => {
// Module: crate::xof_fixed
// Provides: {"impl_89"}
// Dependencies: {}
impl < T : ExtendableOutput + Default , S : ArraySize > Default for XofFixedWrapper < T , S > { fn default () -> Self { Self { hash : Default :: default () , size : PhantomData , } } }
};
}
