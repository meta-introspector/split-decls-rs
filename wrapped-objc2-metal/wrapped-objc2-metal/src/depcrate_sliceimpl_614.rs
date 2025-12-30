// Generated macro for impl_614 (impl)
macro_rules! Depcrate_sliceimpl_614 {
() => {
// Module: crate::slice
// Provides: {"impl_614"}
// Dependencies: {}
# [cfg (all (feature = "MTLRenderCommandEncoder" , feature = "MTLCommandEncoder"))] impl < P : MTLRenderCommandEncoder + objc2 :: Message > MTLRenderCommandEncoderSliceExt for P { unsafe fn setViewports (& self , viewports : & [MTLViewport]) { let (ptr , count) = slice_to_ptr_count (viewports) ; unsafe { self . setViewports_count (ptr , count) } } }
};
}
