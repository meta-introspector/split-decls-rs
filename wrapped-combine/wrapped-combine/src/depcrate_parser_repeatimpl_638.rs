// Generated macro for impl_638 (impl)
macro_rules! Depcrate_parser_repeatimpl_638 {
() => {
// Module: crate::parser::repeat
// Provides: {"impl_638"}
// Dependencies: {}
impl < F , Input , P , S > Parser < Input > for SepEndBy1 < F , P , S > where Input : Stream , F : Extend < P :: Output > + Default , P : Parser < Input > , S : Parser < Input > , { type Output = F ; type PartialState = (Option < Commit < () > > , F , < With < S , Optional < P > > as Parser < Input > > :: PartialState ,) ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , Input :: Error > where M : ParseMode , { let (ref mut parsed_one , ref mut elements , ref mut child_state) = * state ; let rest = match * parsed_one { Some (rest) => rest , None => { let (first , rest) = ctry ! (self . parser . parse_mode (mode , input , & mut child_state . B . state)) ; * parsed_one = Some (rest) ; elements . extend (Some (first)) ; rest } } ; rest . combine_commit (| _ | { let rest = (& mut self . separator) . with (optional (& mut self . parser)) ; let mut iter = Iter :: new (rest , mode , input , child_state) ; elements . extend (iter . by_ref () . scan (() , | _ , x | x)) ; if iter . committed { * parsed_one = Some (Commit :: Commit (())) ; } iter . into_result_fast (elements) . map (| x | { * parsed_one = None ; x }) }) } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { self . parser . add_error (errors) } }
};
}
