// Generated macro for impl_504 (impl)
macro_rules! Depcrate_parser_combinatorimpl_504 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_504"}
// Dependencies: {}
impl < Input , F , O , S > Parser < Input > for Opaque < F , Input , O , S > where Input : Stream , S : Default , F : FnMut (& mut dyn FnMut (& mut dyn Parser < Input , Output = O , PartialState = S >)) , { type Output = O ; type PartialState = S ; fn parse_stream (& mut self , input : & mut Input) -> ParseResult < O , < Input as StreamOnce > :: Error > { let mut x = None ; (self . 0) (& mut | parser | x = Some (parser . parse_stream (input))) ; x . expect ("Parser") } fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < O , < Input as StreamOnce > :: Error > { let mut x = None ; (self . 0) (& mut | parser | x = Some (parser . parse_lazy (input))) ; x . expect ("Parser") } parse_mode ! (Input) ; fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { let mut x = None ; (self . 0) (& mut | parser | { x = Some (if mode . is_first () { parser . parse_first (input , state) } else { parser . parse_partial (input , state) }) }) ; x . expect ("Parser") } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { (self . 0) (& mut | parser | parser . add_error (errors)) ; } fn add_committed_expected_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { (self . 0) (& mut | parser | parser . add_committed_expected_error (errors)) ; } }
};
}
