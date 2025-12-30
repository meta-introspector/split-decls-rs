// Generated macro for impl_247 (impl)
macro_rules! Depcrate_buffer_parimpl_247 {
() => {
// Module: crate::buffer_par
// Provides: {"impl_247"}
// Dependencies: {}
impl < 'a , P > IndexedParallelIterator for PixelsPar < 'a , P > where P : Pixel + Sync + 'a , P :: Subpixel : Sync + 'a , { fn drive < C : Consumer < Self :: Item > > (self , consumer : C) -> C :: Result { self . chunks . map (| v | < P as Pixel > :: from_slice (v)) . drive (consumer) } fn len (& self) -> usize { self . chunks . len () } fn with_producer < CB : ProducerCallback < Self :: Item > > (self , callback : CB) -> CB :: Output { self . chunks . map (| v | < P as Pixel > :: from_slice (v)) . with_producer (callback) } }
};
}
