// Generated macro for impl_421 (impl)
macro_rules! Depcrate_parser_choiceimpl_421 {
() => {
// Module: crate::parser::choice
// Provides: {"impl_421"}
// Dependencies: {}
impl < Input , O , P1 , P2 > Parser < Input > for Or < P1 , P2 > where Input : Stream , P1 : Parser < Input , Output = O > , P2 : Parser < Input , Output = O > , { type Output = O ; type PartialState = < Choice < (P1 , P2) > as Parser < Input > > :: PartialState ; parse_mode ! (Input) ; # [inline] fn parse_mode_impl < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { self . 0 . parse_mode (mode , input , state) } # [inline] fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { if errors . offset != ErrorOffset (0) { self . 0 . add_error (errors) ; } } }
};
}
