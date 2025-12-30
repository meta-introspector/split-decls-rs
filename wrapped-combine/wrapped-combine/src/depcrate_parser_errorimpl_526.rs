// Generated macro for impl_526 (impl)
macro_rules! Depcrate_parser_errorimpl_526 {
() => {
// Module: crate::parser::error
// Provides: {"impl_526"}
// Dependencies: {}
impl < Input , P , S > Parser < Input > for Expected < P , S > where P : Parser < Input > , Input : Stream , S : for < 's > ErrorInfo < 's , Input :: Token , Input :: Range > , { type Output = P :: Output ; type PartialState = P :: PartialState ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { self . 0 . parse_mode (mode , input , state) } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { ParseError :: set_expected (errors , StreamError :: expected (& self . 1) , | errors | { self . 0 . add_error (errors) ; }) } forward_parser ! (Input , parser_count add_committed_expected_error , 0) ; }
};
}
