// Generated macro for impl_258 (impl)
macro_rules! Depcrate_buffer_parimpl_258 {
() => {
// Module: crate::buffer_par
// Provides: {"impl_258"}
// Dependencies: {}
impl < 'a , P > ParallelIterator for EnumeratePixelsMutPar < 'a , P > where P : Pixel + Send + Sync + 'a , P :: Subpixel : Send + Sync + 'a , { type Item = (u32 , u32 , & 'a mut P) ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . pixels . enumerate () . map (| (i , p) | { ((i % self . width as usize) as u32 , (i / self . width as usize) as u32 , p ,) }) . drive_unindexed (consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
