// Generated macro for impl_161 (impl)
macro_rules! Depcrate_astimpl_161 {
() => {
// Module: crate::ast
// Provides: {"impl_161"}
// Dependencies: {}
impl Parse for CallOffset { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (CallOffset , IndexStr < 'b >) > { try_begin_parse ! ("CallOffset" , ctx , input) ; if input . is_empty () { return Err (error :: Error :: UnexpectedEnd) ; } if let Ok (tail) = consume (b"h" , input) { let (offset , tail) = NvOffset :: parse (ctx , subs , tail) ? ; let tail = consume (b"_" , tail) ? ; return Ok ((CallOffset :: NonVirtual (offset) , tail)) ; } if let Ok (tail) = consume (b"v" , input) { let (offset , tail) = VOffset :: parse (ctx , subs , tail) ? ; let tail = consume (b"_" , tail) ? ; return Ok ((CallOffset :: Virtual (offset) , tail)) ; } Err (error :: Error :: UnexpectedText) } }
};
}
