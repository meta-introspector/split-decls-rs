// Generated macro for impl_236 (impl)
macro_rules! Depcrate_framesetterimpl_236 {
() => {
// Module: crate::framesetter
// Provides: {"impl_236"}
// Dependencies: {}
impl CTFramesetter { pub fn new_with_attributed_string (string : CFAttributedStringRef) -> Self { unsafe { let ptr = CTFramesetterCreateWithAttributedString (string) ; CTFramesetter :: wrap_under_create_rule (ptr) } } pub fn create_frame (& self , string_range : CFRange , path : & CGPathRef) -> CTFrame { unsafe { let ptr = CTFramesetterCreateFrame (self . as_concrete_TypeRef () , string_range , path . as_ptr () , null () ,) ; CTFrame :: wrap_under_create_rule (ptr) } } # [doc = " Suggest an appropriate frame size for displaying a text range."] # [doc = ""] # [doc = " Returns a tuple containing an appropriate size (that should be smaller"] # [doc = " than the provided constraints) as well as the range of text that fits in"] # [doc = " this frame."] pub fn suggest_frame_size_with_constraints (& self , string_range : CFRange , frame_attributes : CFDictionaryRef , constraints : CGSize ,) -> (CGSize , CFRange) { unsafe { let mut fit_range = CFRange :: init (0 , 0) ; let size = CTFramesetterSuggestFrameSizeWithConstraints (self . as_concrete_TypeRef () , string_range , frame_attributes , constraints , & mut fit_range ,) ; (size , fit_range) } } }
};
}
