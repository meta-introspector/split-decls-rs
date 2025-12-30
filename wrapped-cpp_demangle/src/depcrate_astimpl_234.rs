// Generated macro for impl_234 (impl)
macro_rules! Depcrate_astimpl_234 {
() => {
// Module: crate::ast
// Provides: {"impl_234"}
// Dependencies: {}
impl Parse for PointerToMemberType { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (PointerToMemberType , IndexStr < 'b >) > { try_begin_parse ! ("PointerToMemberType" , ctx , input) ; let tail = consume (b"M" , input) ? ; let (ty1 , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; let (ty2 , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; Ok ((PointerToMemberType (ty1 , ty2) , tail)) } }
};
}
