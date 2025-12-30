// Generated macro for impl_632 (impl)
macro_rules! Depcrate_parser_repeatimpl_632 {
() => {
// Module: crate::parser::repeat
// Provides: {"impl_632"}
// Dependencies: {}
impl < F , Input , P , S > Parser < Input > for SepBy1 < F , P , S > where Input : Stream , F : Extend < P :: Output > + Default , P : Parser < Input > , S : Parser < Input > , { type Output = F ; type PartialState = (Option < Commit < () > > , F , < With < S , P > as Parser < Input > > :: PartialState ,) ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , Input :: Error > where M : ParseMode , { let (ref mut parsed_one , ref mut elements , ref mut child_state) = * state ; let rest = match * parsed_one { Some (rest) => rest , None => { let (first , rest) = ctry ! (self . parser . parse_mode (mode , input , & mut child_state . B . state)) ; elements . extend (Some (first)) ; rest } } ; rest . combine_commit (move | _ | { let rest = (& mut self . separator) . with (& mut self . parser) ; let mut iter = Iter :: new (rest , mode , input , child_state) ; elements . extend (iter . by_ref ()) ; iter . into_result_fast (elements) . map (| x | { * parsed_one = None ; x }) }) } fn add_committed_expected_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { self . separator . add_error (errors) } forward_parser ! (Input , add_error parser_count , parser) ; }
};
}
