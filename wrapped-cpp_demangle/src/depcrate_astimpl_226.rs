// Generated macro for impl_226 (impl)
macro_rules! Depcrate_astimpl_226 {
() => {
// Module: crate::ast
// Provides: {"impl_226"}
// Dependencies: {}
impl Parse for ArrayType { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (ArrayType , IndexStr < 'b >) > { try_begin_parse ! ("ArrayType" , ctx , input) ; let tail = consume (b"A" , input) ? ; if let Ok ((num , tail)) = parse_number (10 , false , tail) { debug_assert ! (num >= 0) ; let tail = consume (b"_" , tail) ? ; let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; return Ok ((ArrayType :: DimensionNumber (num as _ , ty) , tail)) ; } if let Ok ((expr , tail)) = try_recurse ! (Expression :: parse (ctx , subs , tail)) { let tail = consume (b"_" , tail) ? ; let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; return Ok ((ArrayType :: DimensionExpression (expr , ty) , tail)) ; } let tail = consume (b"_" , tail) ? ; let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; Ok ((ArrayType :: NoDimension (ty) , tail)) } }
};
}
