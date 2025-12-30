// Generated macro for impl_213 (impl)
macro_rules! Depcrate_astimpl_213 {
() => {
// Module: crate::ast
// Provides: {"impl_213"}
// Dependencies: {}
impl Parse for Decltype { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (Decltype , IndexStr < 'b >) > { try_begin_parse ! ("Decltype" , ctx , input) ; let tail = consume (b"D" , input) ? ; if let Ok (tail) = consume (b"t" , tail) { let (expr , tail) = Expression :: parse (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; return Ok ((Decltype :: IdExpression (expr) , tail)) ; } let tail = consume (b"T" , tail) ? ; let (expr , tail) = Expression :: parse (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; Ok ((Decltype :: Expression (expr) , tail)) } }
};
}
