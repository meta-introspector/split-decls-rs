// Generated macro for impl_143 (impl)
macro_rules! Depcrate_astimpl_143 {
() => {
// Module: crate::ast
// Provides: {"impl_143"}
// Dependencies: {}
impl Parse for AbiTag { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (AbiTag , IndexStr < 'b >) > { try_begin_parse ! ("AbiTag" , ctx , input) ; let tail = consume (b"B" , input) ? ; let (source_name , tail) = SourceName :: parse (ctx , subs , tail) ? ; Ok ((AbiTag (source_name) , tail)) } }
};
}
