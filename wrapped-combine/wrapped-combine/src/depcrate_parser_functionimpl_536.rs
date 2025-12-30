// Generated macro for impl_536 (impl)
macro_rules! Depcrate_parser_functionimpl_536 {
() => {
// Module: crate::parser::function
// Provides: {"impl_536"}
// Dependencies: {}
impl < Input , O , F > Parser < Input > for FnParser < Input , F > where Input : Stream , F : FnMut (& mut Input) -> StdParseResult < O , Input > , { type Output = O ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < O , Input :: Error > { (self . 0) (input) . into () } }
};
}
