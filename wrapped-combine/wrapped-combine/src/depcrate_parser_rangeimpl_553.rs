// Generated macro for impl_553 (impl)
macro_rules! Depcrate_parser_rangeimpl_553 {
() => {
// Module: crate::parser::range
// Provides: {"impl_553"}
// Dependencies: {}
impl < Input , P > Parser < Input > for RecognizeWithValue < P > where P : Parser < Input > , Input : RangeStream , < Input as StreamOnce > :: Range : crate :: stream :: Range , { type Output = (< Input as StreamOnce > :: Range , P :: Output) ; type PartialState = (usize , P :: PartialState) ; parse_mode ! (Input) ; # [inline] fn parse_mode < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { let (ref mut distance_state , ref mut child_state) = * state ; let before = input . checkpoint () ; if ! mode . is_first () && input . uncons_range (* distance_state) . is_err () { panic ! ("recognize errored when restoring the input stream to its expected state") ; } let value = match self . 0 . parse_mode (mode , input , child_state) { CommitOk (x) | PeekOk (x) => x , PeekErr (err) => return PeekErr (err) , CommitErr (err) => { * distance_state = input . distance (& before) ; ctry ! (input . reset (before) . committed ()) ; return CommitErr (err) ; } } ; let distance = input . distance (& before) ; ctry ! (input . reset (before) . committed ()) ; take (distance) . parse_lazy (input) . map (| range | { * distance_state = 0 ; (range , value) }) } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { self . 0 . add_error (errors) } }
};
}
