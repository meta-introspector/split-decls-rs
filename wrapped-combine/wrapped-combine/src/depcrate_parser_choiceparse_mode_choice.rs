// Generated macro for parse_mode_choice (macro)
macro_rules! Depcrate_parser_choiceparse_mode_choice {
() => {
// Module: crate::parser::choice
// Provides: {"parse_mode_choice"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! parse_mode_choice { (Input) => { fn parse_partial (& mut self , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce >:: Error > { self . parse_mode_choice ($ crate :: parser :: PartialMode :: default () , input , state) } fn parse_first (& mut self , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , Input :: Error > { self . parse_mode_choice ($ crate :: parser :: FirstMode , input , state) } } ; }
};
}
