// Generated macro for XofFixedWrapper (struct)
macro_rules! Depcrate_xof_fixedXofFixedWrapper {
() => {
// Module: crate::xof_fixed
// Provides: {"XofFixedWrapper"}
// Dependencies: {}
# [doc = " Wrapper around [`ExtendableOutput`] types adding [`OutputSizeUser`] with the given size of `S`."] pub struct XofFixedWrapper < T : ExtendableOutput , S : ArraySize > { hash : T , size : PhantomData < S > , }
};
}
