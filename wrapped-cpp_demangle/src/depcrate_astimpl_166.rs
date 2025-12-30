// Generated macro for impl_166 (impl)
macro_rules! Depcrate_astimpl_166 {
() => {
// Module: crate::ast
// Provides: {"impl_166"}
// Dependencies: {}
impl Parse for VOffset { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (VOffset , IndexStr < 'b >) > { try_begin_parse ! ("VOffset" , ctx , input) ; let (offset , tail) = Number :: parse (ctx , subs , input) ? ; let tail = consume (b"_" , tail) ? ; let (virtual_offset , tail) = Number :: parse (ctx , subs , tail) ? ; Ok ((VOffset (offset , virtual_offset) , tail)) } }
};
}
