// Generated macro for impl_463 (impl)
macro_rules! Depcrate_parser_combinatorimpl_463 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_463"}
// Dependencies: {}
impl < Input , P > Parser < Input > for NoPartial < P > where Input : Stream , P : Parser < Input > , { type Output = < P as Parser < Input > > :: Output ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > { self . 0 . parse_lazy (input) } parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , _mode : M , input : & mut Input , _state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { self . 0 . parse_lazy (input) } forward_parser ! (Input , add_error add_committed_expected_error parser_count , 0) ; }
};
}
