// Generated macro for impl_90 (impl)
macro_rules! Depcrate_astimpl_90 {
() => {
// Module: crate::ast
// Provides: {"impl_90"}
// Dependencies: {}
impl Parse for CloneSuffix { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (CloneSuffix , IndexStr < 'b >) > { try_begin_parse ! ("CloneSuffix" , ctx , input) ; let tail = consume (b"." , input) ? ; let (identifier , mut tail) = CloneTypeIdentifier :: parse (ctx , subs , tail) ? ; let mut numbers = Vec :: with_capacity (1) ; while let Ok ((n , t)) = consume (b"." , tail) . and_then (| t | parse_number (10 , false , t)) { numbers . push (n) ; tail = t ; } let clone_suffix = CloneSuffix (identifier , numbers) ; Ok ((clone_suffix , tail)) } }
};
}
