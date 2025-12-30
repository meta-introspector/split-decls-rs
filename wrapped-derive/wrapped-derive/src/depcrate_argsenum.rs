// Generated macro for Enum (struct)
macro_rules! Depcrate_argsEnum {
() => {
// Module: crate::args
// Provides: {"Enum"}
// Dependencies: {}
# [derive (FromDeriveInput)] # [darling (attributes (graphql) , forward_attrs (doc))] pub struct Enum { pub ident : Ident , pub generics : Generics , pub attrs : Vec < Attribute > , pub data : Data < EnumItem , Ignored > , # [darling (default)] pub internal : bool , # [darling (default)] pub name : Option < String > , # [darling (default)] pub display : bool , # [darling (default)] pub name_type : bool , # [darling (default)] pub rename_items : Option < RenameRule > , # [darling (default)] pub remote : Option < Type > , # [darling (default)] pub visible : Option < Visible > , # [darling (default)] pub inaccessible : bool , # [darling (default , multiple , rename = "tag")] pub tags : Vec < String > , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , # [darling (default , multiple)] pub requires_scopes : Vec < String > , }
};
}
