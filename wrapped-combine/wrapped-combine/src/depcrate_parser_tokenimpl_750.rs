// Generated macro for impl_750 (impl)
macro_rules! Depcrate_parser_tokenimpl_750 {
() => {
// Module: crate::parser::token
// Provides: {"impl_750"}
// Dependencies: {}
impl < Input > Parser < Input > for Eof < Input > where Input : Stream , { type Output = () ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < () , Input :: Error > { let before = input . checkpoint () ; match input . uncons () { Err (ref err) if err . is_unexpected_end_of_input () => PeekOk (()) , _ => { ctry ! (input . reset (before) . committed ()) ; PeekErr (< Input as StreamOnce > :: Error :: empty (input . position ()) . into ()) } } } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { errors . error . add_expected ("end of input") ; } }
};
}
