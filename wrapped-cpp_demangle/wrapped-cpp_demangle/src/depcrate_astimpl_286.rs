// Generated macro for impl_286 (impl)
macro_rules! Depcrate_astimpl_286 {
() => {
// Module: crate::ast
// Provides: {"impl_286"}
// Dependencies: {}
impl Parse for Initializer { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (Initializer , IndexStr < 'b >) > { try_begin_parse ! ("Initializer" , ctx , input) ; let tail = consume (b"pi" , input) ? ; let (exprs , tail) = zero_or_more :: < Expression > (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; Ok ((Initializer (exprs) , tail)) } }
};
}
