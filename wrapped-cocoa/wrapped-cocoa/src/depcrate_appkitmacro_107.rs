// Generated macro for macro_107 (macro)
macro_rules! Depcrate_appkitmacro_107 {
() => {
// Module: crate::appkit
// Provides: {"macro_107"}
// Dependencies: {}
bitflags ! { # [derive (Clone , Copy , Debug , Default , Eq , Hash , Ord , PartialEq , PartialOrd)] pub struct NSAlignmentOptions : c_ulonglong { const NSAlignMinXInward = 1 << 0 ; const NSAlignMinYInward = 1 << 1 ; const NSAlignMaxXInward = 1 << 2 ; const NSAlignMaxYInward = 1 << 3 ; const NSAlignWidthInward = 1 << 4 ; const NSAlignHeightInward = 1 << 5 ; const NSAlignMinXOutward = 1 << 8 ; const NSAlignMinYOutward = 1 << 9 ; const NSAlignMaxXOutward = 1 << 10 ; const NSAlignMaxYOutward = 1 << 11 ; const NSAlignWidthOutward = 1 << 12 ; const NSAlignHeightOutward = 1 << 13 ; const NSAlignMinXNearest = 1 << 16 ; const NSAlignMinYNearest = 1 << 17 ; const NSAlignMaxXNearest = 1 << 18 ; const NSAlignMaxYNearest = 1 << 19 ; const NSAlignWidthNearest = 1 << 20 ; const NSAlignHeightNearest = 1 << 21 ; const NSAlignRectFlipped = 1 << 63 ; const NSAlignAllEdgesInward = NSAlignmentOptions :: NSAlignMinXInward . bits () | NSAlignmentOptions :: NSAlignMaxXInward . bits () | NSAlignmentOptions :: NSAlignMinYInward . bits () | NSAlignmentOptions :: NSAlignMaxYInward . bits () ; const NSAlignAllEdgesOutward = NSAlignmentOptions :: NSAlignMinXOutward . bits () | NSAlignmentOptions :: NSAlignMaxXOutward . bits () | NSAlignmentOptions :: NSAlignMinYOutward . bits () | NSAlignmentOptions :: NSAlignMaxYOutward . bits () ; const NSAlignAllEdgesNearest = NSAlignmentOptions :: NSAlignMinXNearest . bits () | NSAlignmentOptions :: NSAlignMaxXNearest . bits () | NSAlignmentOptions :: NSAlignMinYNearest . bits () | NSAlignmentOptions :: NSAlignMaxYNearest . bits () ; } }
};
}
