// Generated macro for OneofObjectField (struct)
macro_rules! Depcrate_argsOneofObjectField {
() => {
// Module: crate::args
// Provides: {"OneofObjectField"}
// Dependencies: {}
# [derive (FromVariant)] # [darling (attributes (graphql) , forward_attrs (doc))] pub struct OneofObjectField { pub ident : Ident , pub attrs : Vec < Attribute > , pub fields : Fields < syn :: Type > , # [darling (default)] pub name : Option < String > , # [darling (default)] pub validator : Option < Validators > , # [darling (default)] pub visible : Option < Visible > , # [darling (default)] pub inaccessible : bool , # [darling (default , multiple , rename = "tag")] pub tags : Vec < String > , # [darling (default)] pub secret : bool , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , # [darling (default)] pub deprecation : Deprecation , }
};
}
