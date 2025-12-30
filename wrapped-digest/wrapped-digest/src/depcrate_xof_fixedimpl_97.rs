// Generated macro for impl_97 (impl)
macro_rules! Depcrate_xof_fixedimpl_97 {
() => {
// Module: crate::xof_fixed
// Provides: {"impl_97"}
// Dependencies: {}
impl < T : ExtendableOutput + Update , S : ArraySize > Update for XofFixedWrapper < T , S > { fn update (& mut self , data : & [u8]) { self . hash . update (data) } }
};
}
