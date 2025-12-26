use crate::parse_result::{ParseResultBase, ParseResultExt};
use crate::DynMTrackerTrait; // Required for impl ParseResult<T> for OkParse<T> to compile
use std::boxed::Box;

/// Success variant.
pub struct OkParse<T>(pub T);

impl<T> ParseResultBase<T> for OkParse<T> {
    fn handle_failure(&mut self, _tracker: &mut DynMTrackerTrait!()) {}
    fn is_ok(&self) -> bool { true }
    fn unwrap(self) -> Option<T> { Some(self.0) }
}

impl<T> ParseResultExt<T> for OkParse<T> {
    fn map<U, F>(self, op: F) -> Box<dyn ParseResultBase<U>> where F: FnOnce(T) -> U, U: 'static {
        Box::new(OkParse(op(self.0)))
    }
}
