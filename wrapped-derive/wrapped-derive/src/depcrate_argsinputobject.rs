// Generated macro for InputObject (struct)
macro_rules! Depcrate_argsInputObject {
() => {
// Module: crate::args
// Provides: {"InputObject"}
// Dependencies: {}
# [derive (FromDeriveInput)] # [darling (attributes (graphql) , forward_attrs (doc))] pub struct InputObject { pub ident : Ident , pub generics : Generics , pub attrs : Vec < Attribute > , pub data : Data < Ignored , InputObjectField > , # [darling (default)] pub internal : bool , # [darling (default)] pub name : Option < String > , # [darling (default)] pub name_type : bool , # [darling (default)] pub input_name : Option < String > , # [darling (default)] pub rename_fields : Option < RenameRule > , # [darling (default)] pub visible : Option < Visible > , # [darling (default)] pub inaccessible : bool , # [darling (default , multiple , rename = "tag")] pub tags : Vec < String > , # [darling (default , multiple , rename = "concrete")] pub concretes : Vec < ConcreteType > , # [darling (default)] pub validator : Option < Expr > , # [darling (default)] pub complex : bool , # [darling (default)] pub shareable : bool , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , }
};
}
