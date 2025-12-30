// Generated macro for impl_254 (impl)
macro_rules! Depcrate_buffer_parimpl_254 {
() => {
// Module: crate::buffer_par
// Provides: {"impl_254"}
// Dependencies: {}
impl < 'a , P > ParallelIterator for EnumeratePixelsPar < 'a , P > where P : Pixel + Sync + 'a , P :: Subpixel : Sync + 'a , { type Item = (u32 , u32 , & 'a P) ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . pixels . enumerate () . map (| (i , p) | { ((i % self . width as usize) as u32 , (i / self . width as usize) as u32 , p ,) }) . drive_unindexed (consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
