// Generated macro for impl_259 (impl)
macro_rules! Depcrate_buffer_parimpl_259 {
() => {
// Module: crate::buffer_par
// Provides: {"impl_259"}
// Dependencies: {}
impl < 'a , P > IndexedParallelIterator for EnumeratePixelsMutPar < 'a , P > where P : Pixel + Send + Sync + 'a , P :: Subpixel : Send + Sync + 'a , { fn drive < C : Consumer < Self :: Item > > (self , consumer : C) -> C :: Result { self . pixels . enumerate () . map (| (i , p) | { ((i % self . width as usize) as u32 , (i / self . width as usize) as u32 , p ,) }) . drive (consumer) } fn len (& self) -> usize { self . pixels . len () } fn with_producer < CB : ProducerCallback < Self :: Item > > (self , callback : CB) -> CB :: Output { self . pixels . enumerate () . map (| (i , p) | { ((i % self . width as usize) as u32 , (i / self . width as usize) as u32 , p ,) }) . with_producer (callback) } }
};
}
