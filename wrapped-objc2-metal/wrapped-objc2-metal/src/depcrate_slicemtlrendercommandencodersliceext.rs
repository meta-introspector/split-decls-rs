// Generated macro for MTLRenderCommandEncoderSliceExt (trait)
macro_rules! Depcrate_sliceMTLRenderCommandEncoderSliceExt {
() => {
// Module: crate::slice
// Provides: {"MTLRenderCommandEncoderSliceExt"}
// Dependencies: {}
# [cfg (all (feature = "MTLRenderCommandEncoder" , feature = "MTLCommandEncoder"))] pub trait MTLRenderCommandEncoderSliceExt : MTLRenderCommandEncoder + objc2 :: Message { unsafe fn setViewports (& self , viewports : & [MTLViewport]) ; }
};
}
