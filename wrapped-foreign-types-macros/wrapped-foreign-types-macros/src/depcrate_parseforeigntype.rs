// Generated macro for ForeignType (struct)
macro_rules! Depcrate_parseForeignType {
() => {
// Module: crate::parse
// Provides: {"ForeignType"}
// Dependencies: {}
pub struct ForeignType { pub attrs : Vec < Attribute > , pub visibility : Visibility , pub name : Ident , pub generics : Generics , pub oibits : Punctuated < Ident , Token ! [+] > , pub phantom_data : Option < Type > , pub ctype : Type , pub drop : Expr , pub clone : Option < Expr > , }
};
}
