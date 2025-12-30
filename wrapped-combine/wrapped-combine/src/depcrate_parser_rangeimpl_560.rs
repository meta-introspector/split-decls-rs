// Generated macro for impl_560 (impl)
macro_rules! Depcrate_parser_rangeimpl_560 {
() => {
// Module: crate::parser::range
// Provides: {"impl_560"}
// Dependencies: {}
impl < Input , F > Parser < Input > for TakeWhile < Input , F > where Input : RangeStream , Input :: Range : crate :: stream :: Range , F : FnMut (Input :: Token) -> bool , { type Output = Input :: Range ; type PartialState = usize ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { parse_partial_range (mode , input , state , & mut self . 0 , | input , predicate | uncons_while (input , predicate) , | input , predicate | uncons_while (input , predicate) ,) } }
};
}
