// Generated macro for ObjectField (struct)
macro_rules! Depcrate_argsObjectField {
() => {
// Module: crate::args
// Provides: {"ObjectField"}
// Dependencies: {}
# [derive (FromMeta , Default)] # [darling (default)] pub struct ObjectField { pub skip : bool , pub entity : bool , pub name : Option < String > , pub deprecation : Deprecation , pub cache_control : CacheControl , pub external : bool , pub provides : Option < String > , pub requires : Option < String > , pub shareable : bool , pub inaccessible : bool , # [darling (multiple , rename = "tag")] pub tags : Vec < String > , pub override_from : Option < String > , pub guard : Option < Expr > , pub visible : Option < Visible > , pub complexity : Option < Expr > , # [darling (default , multiple)] pub derived : Vec < DerivedField > , pub flatten : bool , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , # [darling (default , multiple)] pub requires_scopes : Vec < String > , }
};
}
