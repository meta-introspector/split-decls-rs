// Generated macro for impl_407 (impl)
macro_rules! Depcrate_parser_choiceimpl_407 {
() => {
// Module: crate::parser::choice
// Provides: {"impl_407"}
// Dependencies: {}
impl < 'a , Input , P > ChoiceParser < Input > for & 'a mut P where Input : Stream , P : ? Sized + ChoiceParser < Input > , { type Output = P :: Output ; type PartialState = P :: PartialState ; parse_mode_choice ! (Input) ; # [inline] fn parse_mode_choice < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { if mode . is_first () { (* * self) . parse_first (input , state) } else { (* * self) . parse_partial (input , state) } } fn add_error_choice (& mut self , error : & mut Tracked < < Input as StreamOnce > :: Error >) { (* * self) . add_error_choice (error) } }
};
}
