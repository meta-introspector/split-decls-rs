// Generated macro for impl_141 (impl)
macro_rules! Depcrate_machinst_vcodeimpl_141 {
() => {
// Module: crate::machinst::vcode
// Provides: {"impl_141"}
// Dependencies: {}
impl VCodeConstantData { # [doc = " Retrieve the constant data as a byte slice."] pub fn as_slice (& self) -> & [u8] { match self { VCodeConstantData :: Pool (_ , d) | VCodeConstantData :: Generated (d) => d . as_slice () , VCodeConstantData :: WellKnown (d) => d , VCodeConstantData :: U64 (value) => & value [..] , } } # [doc = " Calculate the alignment of the constant data."] pub fn alignment (& self) -> u32 { if self . as_slice () . len () <= 8 { 8 } else { 16 } } }
};
}
