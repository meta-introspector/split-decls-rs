// Generated macro for impl_458 (impl)
macro_rules! Depcrate_parser_combinatorimpl_458 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_458"}
// Dependencies: {}
impl < Input , P , F > Parser < Input > for Recognize < F , P > where Input : Stream , P : Parser < Input > , F : Default + Extend < < Input as StreamOnce > :: Token > , { type Output = F ; type PartialState = (F , P :: PartialState) ; parse_mode ! (Input) ; fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { let (ref mut elements , ref mut child_state) = * state ; let before = input . checkpoint () ; let result = self . 0 . parse_mode (mode , input , child_state) ; Self :: recognize_result (elements , before , input , result) } # [inline] fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { self . 0 . add_error (errors) } }
};
}
