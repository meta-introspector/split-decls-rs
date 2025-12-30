// Generated macro for impl_230 (impl)
macro_rules! Depcrate_astimpl_230 {
() => {
// Module: crate::ast
// Provides: {"impl_230"}
// Dependencies: {}
impl Parse for VectorType { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (VectorType , IndexStr < 'b >) > { try_begin_parse ! ("VectorType" , ctx , input) ; let tail = consume (b"Dv" , input) ? ; if let Ok ((num , tail)) = parse_number (10 , false , tail) { debug_assert ! (num >= 0) ; let tail = consume (b"_" , tail) ? ; let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; return Ok ((VectorType :: DimensionNumber (num as _ , ty) , tail)) ; } let (expr , tail) = Expression :: parse (ctx , subs , tail) ? ; let tail = consume (b"_" , tail) ? ; let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; Ok ((VectorType :: DimensionExpression (expr , ty) , tail)) } }
};
}
