// Generated macro for impl_95 (impl)
macro_rules! Depcrate_paramimpl_95 {
() => {
// Module: crate::param
// Provides: {"impl_95"}
// Dependencies: {}
impl < G > MaskSamplingSize for G where G : Unsigned + Sub < U1 > , (Diff < G , U1 > , G) : RangeEncodingSize , { type SampleSize = RangeEncodedPolynomialSize < Diff < G , U1 > , G > ; fn unpack (v : & Array < u8 , Self :: SampleSize >) -> Polynomial { BitPack :: < Diff < G , U1 > , G > :: unpack (v) } }
};
}
