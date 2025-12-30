// Generated macro for impl_566 (impl)
macro_rules! Depcrate_parser_rangeimpl_566 {
() => {
// Module: crate::parser::range
// Provides: {"impl_566"}
// Dependencies: {}
impl < Input > Parser < Input > for TakeUntilRange < Input > where Input : RangeStream , Input :: Range : PartialEq + crate :: stream :: Range , { type Output = Input :: Range ; type PartialState = usize ; # [inline] fn parse_partial (& mut self , input : & mut Input , to_consume : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > { use crate :: stream :: Range ; let len = self . 0 . len () ; let before = input . checkpoint () ; let mut first_stream_error = None ; ctry ! (uncons_range (input , * to_consume)) ; loop { let look_ahead_input = input . checkpoint () ; match input . uncons_range (len) { Ok (xs) => { if xs == self . 0 { let distance = input . distance (& before) - len ; ctry ! (input . reset (before) . committed ()) ; if let Ok (committed) = input . uncons_range (distance) { if distance == 0 { return PeekOk (committed) ; } else { * to_consume = 0 ; return CommitOk (committed) ; } } unreachable ! () ; } else { ctry ! (input . reset (look_ahead_input) . committed ()) ; if input . uncons () . is_err () { unreachable ! () ; } } } Err (first_error) => { if first_stream_error . is_none () { first_stream_error = Some ((first_error , input . distance (& before))) ; } ctry ! (input . reset (look_ahead_input) . committed ()) ; if input . uncons () . is_err () { let (first_error , first_error_distance) = first_stream_error . unwrap () ; ctry ! (input . reset (before) . committed ()) ; * to_consume = first_error_distance ; return wrap_stream_error (input , first_error) ; } } } ; } } }
};
}
