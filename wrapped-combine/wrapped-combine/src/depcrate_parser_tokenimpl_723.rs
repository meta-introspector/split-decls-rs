// Generated macro for impl_723 (impl)
macro_rules! Depcrate_parser_tokenimpl_723 {
() => {
// Module: crate::parser::token
// Provides: {"impl_723"}
// Dependencies: {}
impl < Input , P , R > Parser < Input > for SatisfyMap < Input , P > where Input : Stream , P : FnMut (Input :: Token) -> Option < R > , { type Output = R ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < Self :: Output , Input :: Error > { satisfy_impl (input , & mut self . predicate) } }
};
}
