// Generated macro for impl_735 (impl)
macro_rules! Depcrate_parser_tokenimpl_735 {
() => {
// Module: crate::parser::token
// Provides: {"impl_735"}
// Dependencies: {}
impl < Input > Parser < Input > for Position < Input > where Input : Stream , { type Output = Input :: Position ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < Input :: Position , Input :: Error > { PeekOk (input . position ()) } }
};
}
