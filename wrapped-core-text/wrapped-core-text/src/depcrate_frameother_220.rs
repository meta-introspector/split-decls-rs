// Generated macro for other_220 (other)
macro_rules! Depcrate_frameother_220 {
() => {
// Module: crate::frame
// Provides: {"other_220"}
// Dependencies: {}
# [cfg_attr (feature = "link" , link (name = "CoreText" , kind = "framework"))] extern "C" { fn CTFrameGetTypeID () -> CFTypeID ; fn CTFrameGetLines (frame : CTFrameRef) -> CFArrayRef ; fn CTFrameDraw (frame : CTFrameRef , context : * mut < CGContext as ForeignType > :: CType) ; fn CTFrameGetLineOrigins (frame : CTFrameRef , range : CFRange , origins : * mut CGPoint) ; fn CTFrameGetPath (frame : CTFrameRef) -> SysCGPathRef ; }
};
}
