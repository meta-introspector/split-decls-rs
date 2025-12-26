use rustc_ast::tokenstream::TokenStream;
use rustc_span::Span;
use lib_matcher_loc::MatcherLoc; // From lib-matcher-loc crate
use rustc_parse::parser::Recovery; // From rustc_parse crate
use crate::parse_result::ParseResultBase;

/// Minimal Tracker trait declaration for macro_rules matching.
pub trait LibMacroRuleTracker {
    type Failure;
    /// Static name for debugging/tracing.
    fn description(self: &mut Self) -> &'static str;

    /// Build failure data from mismatched tokens.
    fn build_failure(self: &mut Self, tok: rustc_ast::token::Token, position: u32, msg: &'static str) -> Self::Failure;

    /// Hook before matching a location item.
    fn before_match_loc(self: &mut Self, _loc: lib_matcher_loc::MatcherLoc) {}

    /// Hook after trying a macro arm.
    fn after_arm(self: &mut Self, _matched: bool) {}

    /// Parser recovery strategy on failure.
    fn recovery(self: &mut Self) -> rustc_parse::parser::Recovery;
}
