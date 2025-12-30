// Generated macro for impl_283 (impl)
macro_rules! Depcrate_astimpl_283 {
() => {
// Module: crate::ast
// Provides: {"impl_283"}
// Dependencies: {}
impl Parse for ExprPrimary { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (ExprPrimary , IndexStr < 'b >) > { try_begin_parse ! ("ExprPrimary" , ctx , input) ; let tail = consume (b"L" , input) ? ; if let Ok ((ty , tail)) = try_recurse ! (TypeHandle :: parse (ctx , subs , tail)) { let start = tail . index () ; let num_bytes_in_literal = tail . as_ref () . iter () . take_while (| & & c | c != b'E') . count () ; let tail = tail . range_from (num_bytes_in_literal ..) ; let end = tail . index () ; let tail = consume (b"E" , tail) ? ; let expr = ExprPrimary :: Literal (ty , start , end) ; return Ok ((expr , tail)) ; } let (name , tail) = MangledName :: parse (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; let expr = ExprPrimary :: External (name) ; Ok ((expr , tail)) } }
};
}
