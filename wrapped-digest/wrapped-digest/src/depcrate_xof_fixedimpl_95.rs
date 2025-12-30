// Generated macro for impl_95 (impl)
macro_rules! Depcrate_xof_fixedimpl_95 {
() => {
// Module: crate::xof_fixed
// Provides: {"impl_95"}
// Dependencies: {}
impl < T : ExtendableOutput + KeyInit , S : ArraySize > KeyInit for XofFixedWrapper < T , S > { fn new (key : & crypto_common :: Key < Self >) -> Self { Self { hash : T :: new (key) , size : PhantomData , } } }
};
}
