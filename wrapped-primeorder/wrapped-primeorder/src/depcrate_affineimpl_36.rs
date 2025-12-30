// Generated macro for impl_36 (impl)
macro_rules! Depcrate_affineimpl_36 {
() => {
// Module: crate::affine
// Provides: {"impl_36"}
// Dependencies: {}
impl < C > DecompactPoint < C > for AffinePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , { fn decompact (x_bytes : & FieldBytes < C >) -> CtOption < Self > { Self :: decompress (x_bytes , Choice :: from (0)) . map (| point | point . to_compact ()) } }
};
}
