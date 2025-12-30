// Generated macro for EnumItem (struct)
macro_rules! Depcrate_argsEnumItem {
() => {
// Module: crate::args
// Provides: {"EnumItem"}
// Dependencies: {}
# [derive (FromVariant)] # [darling (attributes (graphql) , forward_attrs (doc))] pub struct EnumItem { pub ident : Ident , pub attrs : Vec < Attribute > , pub fields : Fields < Ignored > , # [darling (default)] pub name : Option < String > , # [darling (default)] pub deprecation : Deprecation , # [darling (default)] pub visible : Option < Visible > , # [darling (default)] pub inaccessible : bool , # [darling (default , multiple , rename = "tag")] pub tags : Vec < String > , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , }
};
}
