// Generated macro for NSRange (struct)
macro_rules! Depcrate_rangeNSRange {
() => {
// Module: crate::range
// Provides: {"NSRange"}
// Dependencies: {}
# [doc = " TODO."] # [doc = ""] # [doc = " See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsrange?language=objc)."] # [repr (C)] # [derive (Clone , Copy , Debug , Default , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct NSRange { # [doc = " The lower bound of the range (inclusive)."] pub location : NSUInteger , # [doc = " The number of items in the range, starting from `location`."] pub length : NSUInteger , }
};
}
