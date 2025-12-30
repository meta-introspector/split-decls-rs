// Generated macro for try_range (function)
macro_rules! Depcrate_spec_parse_functiontry_range {
() => {
// Module: crate::spec::parse::function
// Provides: {"try_range"}
// Dependencies: {}
fn try_range (input : & BStr) -> Option < (& [u8] , spec :: Kind) > { input . strip_prefix (b"...") . map (| rest | (rest , spec :: Kind :: ReachableToMergeBase)) . or_else (| | input . strip_prefix (b"..") . map (| rest | (rest , spec :: Kind :: RangeBetween))) }
};
}
