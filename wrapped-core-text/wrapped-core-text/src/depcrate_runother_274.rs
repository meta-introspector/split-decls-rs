// Generated macro for other_274 (other)
macro_rules! Depcrate_runother_274 {
() => {
// Module: crate::run
// Provides: {"other_274"}
// Dependencies: {}
# [cfg_attr (feature = "link" , link (name = "CoreText" , kind = "framework"))] extern "C" { fn CTRunGetTypeID () -> CFTypeID ; fn CTRunGetAttributes (run : CTRunRef) -> CFDictionaryRef ; fn CTRunGetGlyphCount (run : CTRunRef) -> CFIndex ; fn CTRunGetPositionsPtr (run : CTRunRef) -> * const CGPoint ; fn CTRunGetPositions (run : CTRunRef , range : CFRange , buffer : * const CGPoint) ; fn CTRunGetStringIndicesPtr (run : CTRunRef) -> * const CFIndex ; fn CTRunGetStringIndices (run : CTRunRef , range : CFRange , buffer : * const CFIndex) ; fn CTRunGetGlyphsPtr (run : CTRunRef) -> * const CGGlyph ; fn CTRunGetGlyphs (run : CTRunRef , range : CFRange , buffer : * const CGGlyph) ; fn CTRunGetTypographicBounds (line : CTRunRef , range : CFRange , ascent : * mut CGFloat , descent : * mut CGFloat , leading : * mut CGFloat ,) -> CGFloat ; }
};
}
