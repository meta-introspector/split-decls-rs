// Generated macro for impl_621 (impl)
macro_rules! Depcrate_parser_repeatimpl_621 {
() => {
// Module: crate::parser::repeat
// Provides: {"impl_621"}
// Dependencies: {}
impl < F , Input , P > Parser < Input > for Many1 < F , P > where Input : Stream , F : Extend < P :: Output > + Default , P : Parser < Input > , { type Output = F ; type PartialState = (bool , bool , F , P :: PartialState) ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mut mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < F , Input :: Error > where M : ParseMode , { let (ref mut parsed_one , ref mut committed_state , ref mut elements , ref mut child_state) = * state ; if mode . is_first () || ! * parsed_one { debug_assert ! (!* parsed_one) ; let (first , committed) = ctry ! (self . 0 . parse_mode (mode , input , child_state)) ; elements . extend (Some (first)) ; * committed_state = ! committed . is_peek () ; * parsed_one = true ; mode . set_first () ; } let mut iter = Iter { parser : & mut self . 0 , committed : * committed_state , input , state : State :: Ok , partial_state : child_state , mode , } ; elements . extend (iter . by_ref ()) ; iter . into_result_fast (elements) . map (| x | { * parsed_one = false ; x }) } fn add_committed_expected_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { self . add_error (errors) ; } forward_parser ! (Input , add_error parser_count , 0) ; }
};
}
