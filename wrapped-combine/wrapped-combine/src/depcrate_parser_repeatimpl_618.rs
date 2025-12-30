// Generated macro for impl_618 (impl)
macro_rules! Depcrate_parser_repeatimpl_618 {
() => {
// Module: crate::parser::repeat
// Provides: {"impl_618"}
// Dependencies: {}
impl < F , Input , P > Parser < Input > for Many < F , P > where Input : Stream , P : Parser < Input > , F : Extend < P :: Output > + Default , { type Output = F ; type PartialState = (F , P :: PartialState) ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , Input :: Error > where M : ParseMode , { let (ref mut elements , ref mut child_state) = * state ; let mut iter = (& mut self . 0) . partial_iter (mode , input , child_state) ; elements . extend (iter . by_ref ()) ; iter . into_result_fast (elements) } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { self . 0 . add_error (errors) } fn add_committed_expected_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { self . add_error (errors) ; } fn parser_count (& self) -> ErrorOffset { self . 0 . parser_count () } }
};
}
