// Generated macro for impl_216 (impl)
macro_rules! Depcrate_astimpl_216 {
() => {
// Module: crate::ast
// Provides: {"impl_216"}
// Dependencies: {}
impl Parse for ClassEnumType { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (ClassEnumType , IndexStr < 'b >) > { try_begin_parse ! ("ClassEnumType" , ctx , input) ; if let Ok ((name , tail)) = try_recurse ! (Name :: parse (ctx , subs , input)) { return Ok ((ClassEnumType :: Named (name) , tail)) ; } let tail = consume (b"T" , input) ? ; if let Ok (tail) = consume (b"s" , tail) { let (name , tail) = Name :: parse (ctx , subs , tail) ? ; return Ok ((ClassEnumType :: ElaboratedStruct (name) , tail)) ; } if let Ok (tail) = consume (b"u" , tail) { let (name , tail) = Name :: parse (ctx , subs , tail) ? ; return Ok ((ClassEnumType :: ElaboratedUnion (name) , tail)) ; } let tail = consume (b"e" , tail) ? ; let (name , tail) = Name :: parse (ctx , subs , tail) ? ; Ok ((ClassEnumType :: ElaboratedEnum (name) , tail)) } }
};
}
