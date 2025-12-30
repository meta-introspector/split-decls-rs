// Generated macro for impl_149 (impl)
macro_rules! Depcrate_astimpl_149 {
() => {
// Module: crate::ast
// Provides: {"impl_149"}
// Dependencies: {}
impl Parse for CloneTypeIdentifier { fn parse < 'a , 'b > (ctx : & 'a ParseContext , _subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (CloneTypeIdentifier , IndexStr < 'b >) > { try_begin_parse ! ("CloneTypeIdentifier" , ctx , input) ; if input . is_empty () { return Err (error :: Error :: UnexpectedEnd) ; } let end = input . as_ref () . iter () . map (| & c | c as char) . take_while (| & c | c == '$' || c == '_' || c . is_digit (36)) . count () ; if end == 0 { return Err (error :: Error :: UnexpectedText) ; } let tail = input . range_from (end ..) ; let identifier = CloneTypeIdentifier { start : input . index () , end : tail . index () , } ; Ok ((identifier , tail)) } }
};
}
