// Generated macro for impl_514 (impl)
macro_rules! Depcrate_parser_combinatorimpl_514 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_514"}
// Dependencies: {}
impl < Input , P , Q > Parser < Input > for Spanned < P > where P : Parser < Input > , Input : Stream < Position = Span < Q > > , Q : Ord + Clone , { type Output = P :: Output ; type PartialState = P :: PartialState ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { let start = input . position () . start ; self . 0 . parse_mode (mode , input , state) . map_err (| mut err | { let error_span = err . position () ; if error_span . start == error_span . end { let end = input . position () . end ; err . set_position (Span { start , end }) ; } err }) } forward_parser ! (Input , add_error , add_committed_expected_error , 0) ; }
};
}
