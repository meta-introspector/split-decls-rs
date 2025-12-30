// Generated macro for other_255 (other)
macro_rules! Depcrate_lineother_255 {
() => {
// Module: crate::line
// Provides: {"other_255"}
// Dependencies: {}
# [cfg_attr (feature = "link" , link (name = "CoreText" , kind = "framework"))] extern "C" { fn CTLineGetTypeID () -> CFTypeID ; fn CTLineGetGlyphRuns (line : CTLineRef) -> CFArrayRef ; fn CTLineGetStringRange (line : CTLineRef) -> CFRange ; fn CTLineCreateWithAttributedString (string : CFAttributedStringRef) -> CTLineRef ; fn CTLineDraw (line : CTLineRef , context : * const core_graphics :: sys :: CGContext) ; fn CTLineGetImageBounds (line : CTLineRef , context : * const core_graphics :: sys :: CGContext ,) -> CGRect ; fn CTLineGetTypographicBounds (line : CTLineRef , ascent : * mut CGFloat , descent : * mut CGFloat , leading : * mut CGFloat ,) -> CGFloat ; fn CTLineGetStringIndexForPosition (line : CTLineRef , position : CGPoint) -> CFIndex ; fn CTLineGetOffsetForStringIndex (line : CTLineRef , charIndex : CFIndex , secondaryOffset : * const CGFloat ,) -> CGFloat ; }
};
}
