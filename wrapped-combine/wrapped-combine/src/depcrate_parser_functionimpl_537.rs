// Generated macro for impl_537 (impl)
macro_rules! Depcrate_parser_functionimpl_537 {
() => {
// Module: crate::parser::function
// Provides: {"impl_537"}
// Dependencies: {}
impl < Input , O > Parser < Input > for fn (& mut Input) -> StdParseResult < O , Input > where Input : Stream , { type Output = O ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < O , Input :: Error > { self (input) . into () } }
};
}
