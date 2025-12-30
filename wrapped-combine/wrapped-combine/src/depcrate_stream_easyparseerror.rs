// Generated macro for ParseError (type)
macro_rules! Depcrate_stream_easyParseError {
() => {
// Module: crate::stream::easy
// Provides: {"ParseError"}
// Dependencies: {}
# [doc = " Convenience alias over `Errors` for `StreamOnce` types which makes it possible to specify the"] # [doc = " `Errors` type from a `StreamOnce` by writing `ParseError<Input>` instead of `Errors<Input::Token,"] # [doc = " Input::Range, Input::Position>`"] pub type ParseError < S > = Errors < < S as StreamOnce > :: Token , < S as StreamOnce > :: Range , < S as StreamOnce > :: Position > ;
};
}
