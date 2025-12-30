// Generated macro for impl_254 (impl)
macro_rules! Depcrate_astimpl_254 {
() => {
// Module: crate::ast
// Provides: {"impl_254"}
// Dependencies: {}
impl Parse for TemplateArg { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (TemplateArg , IndexStr < 'b >) > { try_begin_parse ! ("TemplateArg" , ctx , input) ; if let Ok (tail) = consume (b"X" , input) { let (expr , tail) = Expression :: parse (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; return Ok ((TemplateArg :: Expression (expr) , tail)) ; } if let Ok ((expr , tail)) = try_recurse ! (ExprPrimary :: parse (ctx , subs , input)) { return Ok ((TemplateArg :: SimpleExpression (expr) , tail)) ; } if let Ok ((ty , tail)) = try_recurse ! (TypeHandle :: parse (ctx , subs , input)) { return Ok ((TemplateArg :: Type (ty) , tail)) ; } let tail = if input . peek () == Some (b'J') { consume (b"J" , input) ? } else { consume (b"I" , input) ? } ; let (args , tail) = if tail . peek () == Some (b'E') { (vec ! [] , tail) } else { zero_or_more :: < TemplateArg > (ctx , subs , tail) ? } ; let tail = consume (b"E" , tail) ? ; Ok ((TemplateArg :: ArgPack (args) , tail)) } }
};
}
