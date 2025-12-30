// Generated macro for impl_271 (impl)
macro_rules! Depcrate_astimpl_271 {
() => {
// Module: crate::ast
// Provides: {"impl_271"}
// Dependencies: {}
impl Parse for UnresolvedQualifierLevel { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (UnresolvedQualifierLevel , IndexStr < 'b >) > { try_begin_parse ! ("UnresolvedQualifierLevel" , ctx , input) ; let (id , tail) = SimpleId :: parse (ctx , subs , input) ? ; Ok ((UnresolvedQualifierLevel (id) , tail)) } }
};
}
