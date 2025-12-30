// Generated macro for Union (struct)
macro_rules! Depcrate_argsUnion {
() => {
// Module: crate::args
// Provides: {"Union"}
// Dependencies: {}
# [derive (FromDeriveInput)] # [darling (attributes (graphql) , forward_attrs (doc))] pub struct Union { pub ident : Ident , pub generics : Generics , pub attrs : Vec < Attribute > , pub data : Data < UnionItem , Ignored > , # [darling (default)] pub internal : bool , # [darling (default)] pub name : Option < String > , # [darling (default)] pub name_type : bool , # [darling (default)] pub visible : Option < Visible > , # [darling (default)] pub inaccessible : bool , # [darling (default , multiple , rename = "tag")] pub tags : Vec < String > , # [darling (default , multiple , rename = "concrete")] pub concretes : Vec < ConcreteType > , # [darling (default)] pub input_name : Option < String > , }
};
}
