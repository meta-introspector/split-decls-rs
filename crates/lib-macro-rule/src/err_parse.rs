use crate::parse_result::{ParseResultBase, ParseResultExt};
use crate::DynMTrackerTrait; // Assuming Tracker is in lib.rs and pub use'd
use crate::LibMacroRuleTrackerTokenStreamArgT;
use crate::LibMacroRuleTrackerSpanArgT;
use rustc_ast::tokenstream::TokenStream; // for Tracker::build_failure
use rustc_span::Span; // for Tracker::build_failure
use std::boxed::Box; // Required for Box<dyn ParseResult>
use std::marker::PhantomData; // Required for ErrParse

// Placeholder for Failure. This will need proper definition or import.
// Based on lib.rs comments, it might be in `crate::mbe::Failure`.
#[derive(Debug)] // Add derive to satisfy potential debug prints
pub struct Failure {
    // Assuming it needs these fields based on Tracker::build_failure
    pub tokens: TokenStream,
    pub lo: Span,
    pub hi: Span,
}

/// Failure variant with collected diagnostics.
pub struct ErrParse<T> {
    pub failures: Vec<Failure>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ErrParse<T> {
    pub fn new(failures: Vec<Failure>) -> Self {
        ErrParse { failures, _phantom: std::marker::PhantomData }
    }
}

impl<T> ParseResultBase<T> for ErrParse<T> {
    fn handle_failure(&mut self, tracker: &mut DynMTrackerTrait!()) {
        for failure in &self.failures {
            tracker.build_failure(
                rustc_ast::token::Token::dummy(),
                failure.lo.lo().0,
                "macro parsing failure",
            );
        }
    }

    fn is_ok(&self) -> bool { false }
    fn unwrap(self) -> Option<T> { None }
}

impl<T> ParseResultExt<T> for ErrParse<T> {
    fn map<U, F>(self, _op: F) -> Box<dyn ParseResultBase<U>> where F: FnOnce(T) -> U, U: 'static {
        Box::new(ErrParse::new(self.failures))
    }
}
