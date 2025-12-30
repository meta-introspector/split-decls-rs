// Generated macro for impl_parser_for_tuple (macro)
macro_rules! Depcrate_internalimpl_parser_for_tuple {
() => {
// Module: crate::internal
// Provides: {"impl_parser_for_tuple"}
// Dependencies: {}
macro_rules ! impl_parser_for_tuple { ($ ($ parser : ident $ output : ident) ,+) => (# [allow (non_snake_case)] impl < I , $ ($ output) ,+, E : ParseError < I >, $ ($ parser) ,+> Parser < I > for ($ ($ parser) ,+,) where $ ($ parser : Parser < I , Output = $ output , Error = E >) ,+ { type Output = ($ ($ output) ,+,) ; type Error = E ; # [inline (always)] fn process < OM : OutputMode > (& mut self , i : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let ($ (ref mut $ parser) ,+,) = * self ; $ (let (i , $ output) = $ parser . process ::< OutputM < Emit , OM :: Error , OM :: Incomplete >> (i) ?;) + Ok ((i , OM :: Output :: bind (|| ($ ($ output) ,+,)))) } }) }
};
}
