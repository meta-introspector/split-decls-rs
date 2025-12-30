// Generated macro for impl_251 (impl)
macro_rules! Depcrate_buffer_parimpl_251 {
() => {
// Module: crate::buffer_par
// Provides: {"impl_251"}
// Dependencies: {}
impl < 'a , P > IndexedParallelIterator for PixelsMutPar < 'a , P > where P : Pixel + Send + Sync + 'a , P :: Subpixel : Send + Sync + 'a , { fn drive < C : Consumer < Self :: Item > > (self , consumer : C) -> C :: Result { self . chunks . map (| v | < P as Pixel > :: from_slice_mut (v)) . drive (consumer) } fn len (& self) -> usize { self . chunks . len () } fn with_producer < CB : ProducerCallback < Self :: Item > > (self , callback : CB) -> CB :: Output { self . chunks . map (| v | < P as Pixel > :: from_slice_mut (v)) . with_producer (callback) } }
};
}
