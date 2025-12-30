// Generated macro for impl_58 (impl)
macro_rules! Depcrate_linuximpl_58 {
() => {
// Module: crate::linux
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a > Iterator for SegmentIter < 'a > { type Item = Segment < 'a > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| phdr | Segment { phdr : phdr , shlib : PhantomData , }) } }
};
}
