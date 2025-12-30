// Generated macro for forward_deref (macro)
macro_rules! Depcrate_parserforward_deref {
() => {
// Module: crate::parser
// Provides: {"forward_deref"}
// Dependencies: {}
macro_rules ! forward_deref { (Input) => { type Output = P :: Output ; type PartialState = P :: PartialState ; # [inline] fn parse_first (& mut self , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce >:: Error > { (** self) . parse_first (input , state) } # [inline] fn parse_partial (& mut self , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce >:: Error > { (** self) . parse_partial (input , state) } # [inline] fn add_error (& mut self , error : & mut Tracked << Input as StreamOnce >:: Error >) { (** self) . add_error (error) } # [inline] fn add_committed_expected_error (& mut self , error : & mut Tracked << Input as StreamOnce >:: Error >,) { (** self) . add_committed_expected_error (error) } # [inline] fn parser_count (& self) -> ErrorOffset { (** self) . parser_count () } } ; }
};
}
