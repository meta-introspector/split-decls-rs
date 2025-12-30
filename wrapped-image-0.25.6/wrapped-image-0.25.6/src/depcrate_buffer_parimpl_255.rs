// Generated macro for impl_255 (impl)
macro_rules! Depcrate_buffer_parimpl_255 {
() => {
// Module: crate::buffer_par
// Provides: {"impl_255"}
// Dependencies: {}
impl < 'a , P > IndexedParallelIterator for EnumeratePixelsPar < 'a , P > where P : Pixel + Sync + 'a , P :: Subpixel : Sync + 'a , { fn drive < C : Consumer < Self :: Item > > (self , consumer : C) -> C :: Result { self . pixels . enumerate () . map (| (i , p) | { ((i % self . width as usize) as u32 , (i / self . width as usize) as u32 , p ,) }) . drive (consumer) } fn len (& self) -> usize { self . pixels . len () } fn with_producer < CB : ProducerCallback < Self :: Item > > (self , callback : CB) -> CB :: Output { self . pixels . enumerate () . map (| (i , p) | { ((i % self . width as usize) as u32 , (i / self . width as usize) as u32 , p ,) }) . with_producer (callback) } }
};
}
