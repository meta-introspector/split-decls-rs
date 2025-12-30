// Generated macro for impl_117 (impl)
macro_rules! Depcrate_unsupportedimpl_117 {
() => {
// Module: crate::unsupported
// Provides: {"impl_117"}
// Dependencies: {}
impl < 'a > SharedLibraryTrait for SharedLibrary < 'a > { type Segment = Segment < 'a > ; type SegmentIter = SegmentIter < 'a > ; # [inline] fn name (& self) -> & OsStr { unreachable ! () } fn id (& self) -> Option < SharedLibraryId > { unreachable ! () } fn segments (& self) -> Self :: SegmentIter { SegmentIter { phantom : PhantomData , } } # [inline] fn virtual_memory_bias (& self) -> Bias { unreachable ! () } fn each < F , C > (_f : F) where F : FnMut (& Self) -> C , C : Into < IterationControl > , { } }
};
}
