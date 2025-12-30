// Generated macro for impl_250 (impl)
macro_rules! Depcrate_astimpl_250 {
() => {
// Module: crate::ast
// Provides: {"impl_250"}
// Dependencies: {}
impl Parse for TemplateArgs { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (TemplateArgs , IndexStr < 'b >) > { try_begin_parse ! ("TemplateArgs" , ctx , input) ; let tail = consume (b"I" , input) ? ; let (args , tail) = one_or_more :: < TemplateArg > (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; Ok ((TemplateArgs (args) , tail)) } }
};
}
