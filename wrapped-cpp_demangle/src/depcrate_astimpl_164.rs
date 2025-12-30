// Generated macro for impl_164 (impl)
macro_rules! Depcrate_astimpl_164 {
() => {
// Module: crate::ast
// Provides: {"impl_164"}
// Dependencies: {}
impl Parse for NvOffset { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (NvOffset , IndexStr < 'b >) > { try_begin_parse ! ("NvOffset" , ctx , input) ; Number :: parse (ctx , subs , input) . map (| (num , tail) | (NvOffset (num) , tail)) } }
};
}
