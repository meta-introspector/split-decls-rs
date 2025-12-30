// Generated macro for impl_246 (impl)
macro_rules! Depcrate_buffer_parimpl_246 {
() => {
// Module: crate::buffer_par
// Provides: {"impl_246"}
// Dependencies: {}
impl < 'a , P > ParallelIterator for PixelsPar < 'a , P > where P : Pixel + Sync + 'a , P :: Subpixel : Sync + 'a , { type Item = & 'a P ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . chunks . map (| v | < P as Pixel > :: from_slice (v)) . drive_unindexed (consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
