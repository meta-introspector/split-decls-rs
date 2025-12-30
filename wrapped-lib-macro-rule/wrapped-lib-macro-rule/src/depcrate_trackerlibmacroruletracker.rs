// Generated macro for LibMacroRuleTracker (trait)
macro_rules! Depcrate_trackerLibMacroRuleTracker {
() => {
// Module: crate::tracker
// Provides: {"LibMacroRuleTracker"}
// Dependencies: {}
# [doc = " Minimal Tracker trait declaration for macro_rules matching."] pub trait LibMacroRuleTracker { type Failure ; # [doc = " Static name for debugging/tracing."] fn description (self : & mut Self) -> & 'static str ; # [doc = " Build failure data from mismatched tokens."] fn build_failure (self : & mut Self , tok : rustc_ast :: token :: Token , position : u32 , msg : & 'static str) -> Self :: Failure ; # [doc = " Hook before matching a location item."] fn before_match_loc (self : & mut Self , _loc : lib_matcher_loc :: MatcherLoc) { } # [doc = " Hook after trying a macro arm."] fn after_arm (self : & mut Self , _matched : bool) { } # [doc = " Parser recovery strategy on failure."] fn recovery (self : & mut Self) -> rustc_parse :: parser :: Recovery ; }
};
}
