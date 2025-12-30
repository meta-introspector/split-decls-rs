// Generated macro for impl_154 (impl)
macro_rules! Depcrate_astimpl_154 {
() => {
// Module: crate::ast
// Provides: {"impl_154"}
// Dependencies: {}
impl Parse for SeqId { fn parse < 'a , 'b > (ctx : & 'a ParseContext , _subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (SeqId , IndexStr < 'b >) > { try_begin_parse ! ("SeqId" , ctx , input) ; parse_number (36 , false , input) . map (| (num , tail) | (SeqId (num as _) , tail)) } }
};
}
