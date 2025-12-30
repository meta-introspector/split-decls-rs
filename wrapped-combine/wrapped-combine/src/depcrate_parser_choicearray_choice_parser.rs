// Generated macro for array_choice_parser (macro)
macro_rules! Depcrate_parser_choicearray_choice_parser {
() => {
// Module: crate::parser::choice
// Provides: {"array_choice_parser"}
// Dependencies: {}
macro_rules ! array_choice_parser { ($ ($ t : tt) +) => { $ (impl < Input , P > ChoiceParser < Input > for [P ; $ t] where Input : Stream , P : Parser < Input >, { type Output = P :: Output ; type PartialState = < [P] as ChoiceParser < Input >>:: PartialState ; parse_mode_choice ! (Input) ; # [inline] fn parse_mode_choice < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce >:: Error > where M : ParseMode , { if mode . is_first () { self [..] . parse_first (input , state) } else { self [..] . parse_partial (input , state) } } fn add_error_choice (& mut self , error : & mut Tracked << Input as StreamOnce >:: Error >) { self [..] . add_error_choice (error) } }) + } ; }
};
}
