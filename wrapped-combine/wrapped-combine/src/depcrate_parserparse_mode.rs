// Generated macro for parse_mode (macro)
macro_rules! Depcrate_parserparse_mode {
() => {
// Module: crate::parser
// Provides: {"parse_mode"}
// Dependencies: {}
# [doc = " Internal API. May break without a semver bump"] # [macro_export] # [doc (hidden)] macro_rules ! parse_mode { ($ input_type : ty) => { # [inline] fn parse_partial (& mut self , input : & mut $ input_type , state : & mut Self :: PartialState ,) -> $ crate :: error :: ParseResult < Self :: Output , <$ input_type as $ crate :: StreamOnce >:: Error > { self . parse_mode ($ crate :: parser :: PartialMode :: default () , input , state) } # [inline] fn parse_first (& mut self , input : & mut $ input_type , state : & mut Self :: PartialState ,) -> $ crate :: error :: ParseResult < Self :: Output , <$ input_type as $ crate :: StreamOnce >:: Error > { self . parse_mode ($ crate :: parser :: FirstMode , input , state) } } ; }
};
}
