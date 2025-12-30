// Generated macro for parse_partial_range (function)
macro_rules! Depcrate_parser_rangeparse_partial_range {
() => {
// Module: crate::parser::range
// Provides: {"parse_partial_range"}
// Dependencies: {}
# [inline] fn parse_partial_range < M , F , G , S , Input > (mode : M , input : & mut Input , distance_state : & mut usize , state : S , first : F , resume : G ,) -> ParseResult < Input :: Range , Input :: Error > where M : ParseMode , F : FnOnce (& mut Input , S) -> ParseResult < Input :: Range , < Input as StreamOnce > :: Error > , G : FnOnce (& mut Input , S) -> ParseResult < Input :: Range , < Input as StreamOnce > :: Error > , Input : RangeStream , { let before = input . checkpoint () ; if ! input . is_partial () { first (input , state) } else if mode . is_first () || * distance_state == 0 { let result = first (input , state) ; if let CommitErr (_) = result { * distance_state = input . distance (& before) ; ctry ! (input . reset (before) . committed ()) ; } result } else { if input . uncons_range (* distance_state) . is_err () { panic ! ("recognize errored when restoring the input stream to its expected state") ; } match resume (input , state) { CommitOk (_) | PeekOk (_) => () , PeekErr (err) => return PeekErr (err) , CommitErr (err) => { * distance_state = input . distance (& before) ; ctry ! (input . reset (before) . committed ()) ; return CommitErr (err) ; } } let distance = input . distance (& before) ; ctry ! (input . reset (before) . committed ()) ; take (distance) . parse_lazy (input) . map (| range | { * distance_state = 0 ; range }) } }
};
}
