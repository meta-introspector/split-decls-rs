// Generated macro for impl_416 (impl)
macro_rules! Depcrate_parser_choiceimpl_416 {
() => {
// Module: crate::parser::choice
// Provides: {"impl_416"}
// Dependencies: {}
impl < Input , P > Parser < Input > for Choice < P > where Input : Stream , P : ChoiceParser < Input > , { type Output = P :: Output ; type PartialState = P :: PartialState ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { self . 0 . parse_mode_choice (mode , input , state) } fn add_error (& mut self , error : & mut Tracked < < Input as StreamOnce > :: Error >) { let before = error . offset . 0 ; self . 0 . add_error_choice (error) ; error . offset . 0 = before . saturating_sub (1) ; } }
};
}
