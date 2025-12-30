// Generated macro for impl_102 (impl)
macro_rules! Depcrate_astimpl_102 {
() => {
// Module: crate::ast
// Provides: {"impl_102"}
// Dependencies: {}
impl Parse for UnscopedName { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (UnscopedName , IndexStr < 'b >) > { try_begin_parse ! ("UnscopedName" , ctx , input) ; if let Ok (tail) = consume (b"St" , input) { let (name , tail) = UnqualifiedName :: parse (ctx , subs , tail) ? ; return Ok ((UnscopedName :: Std (name) , tail)) ; } let (name , tail) = UnqualifiedName :: parse (ctx , subs , input) ? ; Ok ((UnscopedName :: Unqualified (name) , tail)) } }
};
}
