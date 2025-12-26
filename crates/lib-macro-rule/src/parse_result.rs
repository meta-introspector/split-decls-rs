use crate::MTrackerTrait;
use crate::DynMTrackerTrait;

use std::boxed::Box;
// use rustc_ast::tokenstream::TokenStream; // Not needed directly here
// use rustc_span::Span; // Not needed directly here
use std::marker::PhantomData; // For ErrParse if needed by OkParse/ErrParse in their implementation of these traits


pub trait ParseResultBase<T> {
    fn handle_failure(&mut self, tracker: &mut DynMTrackerTrait!());
    fn is_ok(&self) -> bool;
    fn unwrap(self) -> Option<T>;
}

// This trait will contain methods that are not object safe
pub trait ParseResultExt<T>: Sized { // Sized is required for blanket impl
    fn map<U, F>(self, op: F) -> Box<dyn ParseResultBase<U>>
    where F: FnOnce(T) -> U, U: 'static;
}
