// Generated macro for impl_466 (impl)
macro_rules! Depcrate_parser_combinatorimpl_466 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_466"}
// Dependencies: {}
impl < Input , P > Parser < Input > for Ignore < P > where Input : Stream , P : Parser < Input > , { type Output = () ; type PartialState = P :: PartialState ; # [inline] fn parse_lazy (& mut self , input : & mut Input ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > { self . 0 . parse_lazy (input) . map (| _ | ()) } parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { self . 0 . parse_mode (mode , input , state) . map (| _ | ()) } forward_parser ! (Input , add_error add_committed_expected_error parser_count , 0) ; }
};
}
