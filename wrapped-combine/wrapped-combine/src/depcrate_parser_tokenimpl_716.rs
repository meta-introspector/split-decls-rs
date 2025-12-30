// Generated macro for impl_716 (impl)
macro_rules! Depcrate_parser_tokenimpl_716 {
() => {
// Module: crate::parser::token
// Provides: {"impl_716"}
// Dependencies: {}
impl < Input > Parser < Input > for Any < Input > where Input : Stream , { type Output = Input :: Token ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < Input :: Token , Input :: Error > { uncons (input) } }
};
}
