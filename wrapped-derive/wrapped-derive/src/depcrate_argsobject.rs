// Generated macro for Object (struct)
macro_rules! Depcrate_argsObject {
() => {
// Module: crate::args
// Provides: {"Object"}
// Dependencies: {}
# [derive (FromMeta , Default)] # [darling (default)] pub struct Object { pub internal : bool , pub name : Option < String > , pub name_type : bool , pub rename_fields : Option < RenameRule > , pub rename_args : Option < RenameRule > , pub cache_control : CacheControl , pub extends : bool , pub shareable : bool , pub inaccessible : bool , pub interface_object : bool , # [darling (multiple , rename = "tag")] pub tags : Vec < String > , pub use_type_description : bool , pub visible : Option < Visible > , pub serial : bool , # [darling (default , rename = "unresolvable")] pub resolvability : Resolvability , # [darling (multiple , rename = "concrete")] pub concretes : Vec < ConcreteType > , # [darling (default)] pub guard : Option < Expr > , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , # [darling (default , multiple)] pub requires_scopes : Vec < String > , }
};
}
