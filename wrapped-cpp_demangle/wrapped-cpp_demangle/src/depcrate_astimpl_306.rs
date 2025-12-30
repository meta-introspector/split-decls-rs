// Generated macro for impl_306 (impl)
macro_rules! Depcrate_astimpl_306 {
() => {
// Module: crate::ast
// Provides: {"impl_306"}
// Dependencies: {}
impl Parse for DataMemberPrefix { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (DataMemberPrefix , IndexStr < 'b >) > { try_begin_parse ! ("DataMemberPrefix" , ctx , input) ; let (name , tail) = SourceName :: parse (ctx , subs , input) ? ; let tail = consume (b"M" , tail) ? ; Ok ((DataMemberPrefix (name) , tail)) } }
};
}
