// Generated macro for impl_250 (impl)
macro_rules! Depcrate_buffer_parimpl_250 {
() => {
// Module: crate::buffer_par
// Provides: {"impl_250"}
// Dependencies: {}
impl < 'a , P > ParallelIterator for PixelsMutPar < 'a , P > where P : Pixel + Send + Sync + 'a , P :: Subpixel : Send + Sync + 'a , { type Item = & 'a mut P ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . chunks . map (| v | < P as Pixel > :: from_slice_mut (v)) . drive_unindexed (consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
