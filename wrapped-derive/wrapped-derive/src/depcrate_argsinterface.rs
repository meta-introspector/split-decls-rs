// Generated macro for Interface (struct)
macro_rules! Depcrate_argsInterface {
() => {
// Module: crate::args
// Provides: {"Interface"}
// Dependencies: {}
# [derive (FromDeriveInput)] # [darling (attributes (graphql) , forward_attrs (doc))] pub struct Interface { pub ident : Ident , pub generics : Generics , pub attrs : Vec < Attribute > , pub data : Data < InterfaceMember , Ignored > , # [darling (default)] pub internal : bool , # [darling (default)] pub name : Option < String > , # [darling (default)] pub name_type : bool , # [darling (default)] pub rename_fields : Option < RenameRule > , # [darling (default)] pub rename_args : Option < RenameRule > , # [darling (default , multiple , rename = "field")] pub fields : Vec < InterfaceField > , # [darling (default)] pub extends : bool , # [darling (default)] pub visible : Option < Visible > , # [darling (default)] pub inaccessible : bool , # [darling (default , multiple , rename = "tag")] pub tags : Vec < String > , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , # [darling (default)] pub input_name : Option < String > , # [darling (default , multiple)] pub requires_scopes : Vec < String > , }
};
}
