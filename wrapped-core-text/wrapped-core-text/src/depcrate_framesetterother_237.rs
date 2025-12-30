// Generated macro for other_237 (other)
macro_rules! Depcrate_framesetterother_237 {
() => {
// Module: crate::framesetter
// Provides: {"other_237"}
// Dependencies: {}
# [cfg_attr (feature = "link" , link (name = "CoreText" , kind = "framework"))] extern "C" { fn CTFramesetterGetTypeID () -> CFTypeID ; fn CTFramesetterCreateWithAttributedString (string : CFAttributedStringRef) -> CTFramesetterRef ; fn CTFramesetterCreateFrame (framesetter : CTFramesetterRef , string_range : CFRange , path : * mut < CGPath as ForeignType > :: CType , attributes : * const core :: ffi :: c_void ,) -> CTFrameRef ; fn CTFramesetterSuggestFrameSizeWithConstraints (framesetter : CTFramesetterRef , string_range : CFRange , frame_attributes : CFDictionaryRef , constraints : CGSize , fitRange : * mut CFRange ,) -> CGSize ; }
};
}
