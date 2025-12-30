// Generated macro for impl_152 (impl)
macro_rules! Depcrate_astimpl_152 {
() => {
// Module: crate::ast
// Provides: {"impl_152"}
// Dependencies: {}
impl Parse for Number { fn parse < 'a , 'b > (ctx : & 'a ParseContext , _subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (isize , IndexStr < 'b >) > { try_begin_parse ! ("Number" , ctx , input) ; parse_number (10 , true , input) } }
};
}
