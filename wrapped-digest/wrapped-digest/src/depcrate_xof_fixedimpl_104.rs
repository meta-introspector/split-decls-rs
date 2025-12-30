// Generated macro for impl_104 (impl)
macro_rules! Depcrate_xof_fixedimpl_104 {
() => {
// Module: crate::xof_fixed
// Provides: {"impl_104"}
// Dependencies: {}
impl < T : ExtendableOutput + CustomizedInit , S : ArraySize > CustomizedInit for XofFixedWrapper < T , S > { fn new_customized (customization : & [u8]) -> Self { Self { hash : T :: new_customized (customization) , size : PhantomData , } } }
};
}
