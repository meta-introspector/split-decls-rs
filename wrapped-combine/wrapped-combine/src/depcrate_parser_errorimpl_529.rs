// Generated macro for impl_529 (impl)
macro_rules! Depcrate_parser_errorimpl_529 {
() => {
// Module: crate::parser::error
// Provides: {"impl_529"}
// Dependencies: {}
impl < Input , P > Parser < Input > for Silent < P > where P : Parser < Input > , Input : Stream , { type Output = P :: Output ; type PartialState = P :: PartialState ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { self . 0 . parse_mode (mode , input , state) . map_err (| mut err | { err . clear_expected () ; err }) } fn add_error (& mut self , _errors : & mut Tracked < < Input as StreamOnce > :: Error >) { } fn add_committed_expected_error (& mut self , _errors : & mut Tracked < < Input as StreamOnce > :: Error > ,) { } forward_parser ! (Input , parser_count , 0) ; }
};
}
