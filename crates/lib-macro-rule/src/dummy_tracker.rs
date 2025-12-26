use std::boxed::Box; // Required for Box<dyn ParseResult>
use rustc_ast::tokenstream::TokenStream;
use rustc_parse::parser::Recovery;
use rustc_span::Span;
use lib_matcher_loc::MatcherLoc; // From lib-matcher-loc crate
use crate::MTrackerTrait; // From our modules
use crate::ImplMTrackerTrait;
use crate::parse_result::ParseResultBase;
use crate::err_parse::ErrParse;

#[derive(Clone, Copy)]
pub struct DummyTracker;

ImplMTrackerTrait!{DummyTracker {
    type Failure = Box<dyn ParseResultBase<()>>;
    fn description(self: &mut Self) -> &'static str {
        "dummy"
    }

    fn build_failure(
        self: &mut Self,
        _tok: rustc_ast::token::Token,
        _position: u32,
        _msg: &'static str,
    ) -> Self::Failure {
        Box::new(ErrParse::new(vec![]))
    }

    fn before_match_loc(self: &mut Self, _loc: lib_matcher_loc::MatcherLoc) {}
    fn after_arm(self: &mut Self, _matched: bool) {}
    fn recovery(self: &mut Self) -> rustc_parse::parser::Recovery {
        rustc_parse::parser::Recovery::Forbidden
    }
}}
