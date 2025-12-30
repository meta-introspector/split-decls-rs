// Generated macro for parse_mode_dispatch (macro)
macro_rules! Depcrate_parser_choiceparse_mode_dispatch {
() => {
// Module: crate::parser::choice
// Provides: {"parse_mode_dispatch"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! parse_mode_dispatch { () => { fn parse_partial (& mut self , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce >:: Error > { self . parse_mode_dispatch ($ crate :: parser :: PartialMode :: default () , input , state) } fn parse_first (& mut self , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce >:: Error > { self . parse_mode_dispatch ($ crate :: parser :: FirstMode , input , state) } } ; }
};
}
