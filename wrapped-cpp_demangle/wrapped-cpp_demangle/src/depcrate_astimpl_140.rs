// Generated macro for impl_140 (impl)
macro_rules! Depcrate_astimpl_140 {
() => {
// Module: crate::ast
// Provides: {"impl_140"}
// Dependencies: {}
impl Parse for AbiTags { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (AbiTags , IndexStr < 'b >) > { try_begin_parse ! ("AbiTags" , ctx , input) ; let (tags , tail) = zero_or_more :: < AbiTag > (ctx , subs , input) ? ; Ok ((AbiTags (tags) , tail)) } }
};
}
