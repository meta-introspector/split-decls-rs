// Generated macro for impl_247 (impl)
macro_rules! Depcrate_astimpl_247 {
() => {
// Module: crate::ast
// Provides: {"impl_247"}
// Dependencies: {}
impl Parse for FunctionParam { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (FunctionParam , IndexStr < 'b >) > { try_begin_parse ! ("FunctionParam" , ctx , input) ; let tail = consume (b"f" , input) ? ; if tail . is_empty () { return Err (error :: Error :: UnexpectedEnd) ; } let (scope , tail) = if let Ok (tail) = consume (b"L" , tail) { parse_number (10 , false , tail) ? } else { (0 , tail) } ; let tail = consume (b"p" , tail) ? ; let (qualifiers , tail) = CvQualifiers :: parse (ctx , subs , tail) ? ; let (param , tail) = if tail . peek () == Some (b'T') { (None , consume (b"T" , tail) ?) } else if let Ok ((num , tail)) = parse_number (10 , false , tail) { (Some (num as usize + 1) , consume (b"_" , tail) ?) } else { (Some (0) , consume (b"_" , tail) ?) } ; Ok ((FunctionParam (scope as _ , qualifiers , param) , tail)) } }
};
}
