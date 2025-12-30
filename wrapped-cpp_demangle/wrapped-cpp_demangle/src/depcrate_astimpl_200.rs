// Generated macro for impl_200 (impl)
macro_rules! Depcrate_astimpl_200 {
() => {
// Module: crate::ast
// Provides: {"impl_200"}
// Dependencies: {}
impl Parse for ExceptionSpec { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (ExceptionSpec , IndexStr < 'b >) > { try_begin_parse ! ("ExceptionSpec" , ctx , input) ; if let Ok (tail) = consume (b"Do" , input) { return Ok ((ExceptionSpec :: NoExcept , tail)) ; } let tail = consume (b"DO" , input) ? ; let (expr , tail) = Expression :: parse (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; Ok ((ExceptionSpec :: Computed (expr) , tail)) } }
};
}
