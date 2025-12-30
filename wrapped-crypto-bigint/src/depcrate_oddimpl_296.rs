// Generated macro for impl_296 (impl)
macro_rules! Depcrate_oddimpl_296 {
() => {
// Module: crate::odd
// Provides: {"impl_296"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Resize for & Odd < BoxedUint > { type Output = Odd < BoxedUint > ; fn resize_unchecked (self , at_least_bits_precision : u32) -> Self :: Output { Odd ((& self . 0) . resize_unchecked (at_least_bits_precision)) } fn try_resize (self , at_least_bits_precision : u32) -> Option < Self :: Output > { (& self . 0) . try_resize (at_least_bits_precision) . map (Odd) } }
};
}
