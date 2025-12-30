// Generated macro for Accumulator (struct)
macro_rules! Depcrate_errorAccumulator {
() => {
// Module: crate::error
// Provides: {"Accumulator"}
// Dependencies: {}
# [doc = " Accumulator for errors, for helping call [`Error::multiple`]."] # [doc = ""] # [doc = " See the docs for [`darling::Error`](Error) for more discussion of error handling with darling."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " `Accumulator` panics on drop unless [`finish`](Self::finish), [`finish_with`](Self::finish_with),"] # [doc = " or [`into_inner`](Self::into_inner) has been called, **even if it contains no errors**."] # [doc = " If you want to discard an `Accumulator` that you know to be empty, use `accumulator.finish().unwrap()`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate darling_core as darling;"] # [doc = " # struct Thing;"] # [doc = " # struct Output;"] # [doc = " # impl Thing { fn validate(self) -> darling::Result<Output> { Ok(Output) } }"] # [doc = " fn validate_things(inputs: Vec<Thing>) -> darling::Result<Vec<Output>> {"] # [doc = "     let mut errors = darling::Error::accumulator();"] # [doc = ""] # [doc = "     let outputs = inputs"] # [doc = "         .into_iter()"] # [doc = "         .filter_map(|thing| errors.handle_in(|| thing.validate()))"] # [doc = "         .collect::<Vec<_>>();"] # [doc = ""] # [doc = "     errors.finish()?;"] # [doc = "     Ok(outputs)"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] # [must_use = "Accumulator will panic on drop if not defused."] pub struct Accumulator (Option < Vec < Error > >) ;
};
}
