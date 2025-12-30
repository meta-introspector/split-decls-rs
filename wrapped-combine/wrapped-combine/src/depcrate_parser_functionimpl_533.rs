// Generated macro for impl_533 (impl)
macro_rules! Depcrate_parser_functionimpl_533 {
() => {
// Module: crate::parser::function
// Provides: {"impl_533"}
// Dependencies: {}
impl < 'a , Input : Stream , O > Parser < Input > for dyn FnMut (& mut Input) -> StdParseResult < O , Input > + 'a { type Output = O ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < O , Input :: Error > { self (input) . into () } }
};
}
