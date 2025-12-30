// Generated macro for Pattern (struct)
macro_rules! Depcrate_provider_pattern_reference_patternPattern {
() => {
// Module: crate::provider::pattern::reference::pattern
// Provides: {"Pattern"}
// Dependencies: {}
# [doc = " A fully-owned, non-zero-copy type corresponding to [`Pattern`](super::super::runtime::Pattern)."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , Default , Clone , PartialEq)] pub struct Pattern { pub (crate) items : Vec < PatternItem > , pub (crate) time_granularity : TimeGranularity , }
};
}
