// Generated macro for impl_303 (impl)
macro_rules! Depcrate_astimpl_303 {
() => {
// Module: crate::ast
// Provides: {"impl_303"}
// Dependencies: {}
impl Parse for LambdaSig { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (LambdaSig , IndexStr < 'b >) > { try_begin_parse ! ("LambdaSig" , ctx , input) ; let (types , tail) = if let Ok (tail) = consume (b"v" , input) { (vec ! [] , tail) } else { one_or_more :: < TypeHandle > (ctx , subs , input) ? } ; Ok ((LambdaSig (types) , tail)) } }
};
}
