// Generated macro for impl_209 (impl)
macro_rules! Depcrate_astimpl_209 {
() => {
// Module: crate::ast
// Provides: {"impl_209"}
// Dependencies: {}
impl Parse for BareFunctionType { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (BareFunctionType , IndexStr < 'b >) > { try_begin_parse ! ("BareFunctionType" , ctx , input) ; let (types , tail) = one_or_more :: < TypeHandle > (ctx , subs , input) ? ; Ok ((BareFunctionType (types) , tail)) } }
};
}
