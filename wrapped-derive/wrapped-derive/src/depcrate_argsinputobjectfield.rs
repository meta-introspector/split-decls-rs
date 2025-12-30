// Generated macro for InputObjectField (struct)
macro_rules! Depcrate_argsInputObjectField {
() => {
// Module: crate::args
// Provides: {"InputObjectField"}
// Dependencies: {}
# [derive (FromField)] # [darling (attributes (graphql) , forward_attrs (doc))] pub struct InputObjectField { pub ident : Option < Ident > , pub ty : Type , pub vis : Visibility , pub attrs : Vec < Attribute > , # [darling (default)] pub name : Option < String > , # [darling (default)] pub default : Option < DefaultValue > , # [darling (default)] pub default_with : Option < LitStr > , # [darling (default)] pub validator : Option < Validators > , # [darling (default)] pub flatten : bool , # [darling (default)] pub skip : bool , # [darling (default)] pub skip_input : bool , # [darling (default)] pub process_with : Option < Expr > , # [darling (default)] pub skip_output : bool , # [darling (default)] pub visible : Option < Visible > , # [darling (default)] pub inaccessible : bool , # [darling (default , multiple , rename = "tag")] pub tags : Vec < String > , # [darling (default)] pub secret : bool , # [darling (default)] pub shareable : bool , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , # [darling (default)] pub deprecation : Deprecation , }
};
}
