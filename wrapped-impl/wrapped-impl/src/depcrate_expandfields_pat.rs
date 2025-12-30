// Generated macro for fields_pat (function)
macro_rules! Depcrate_expandfields_pat {
() => {
// Module: crate::expand
// Provides: {"fields_pat"}
// Dependencies: {}
fn fields_pat (fields : & [Field]) -> TokenStream { let mut members = fields . iter () . map (| field | & field . member) . peekable () ; match members . peek () { Some (MemberUnraw :: Named (_)) => quote ! ({ # (# members) ,* }) , Some (MemberUnraw :: Unnamed (_)) => { let vars = members . map (| member | match member { MemberUnraw :: Unnamed (index) => format_ident ! ("_{}" , index) , MemberUnraw :: Named (_) => unreachable ! () , }) ; quote ! ((# (# vars) ,*)) } None => quote ! ({ }) , } }
};
}
