// Generated macro for Subscription (struct)
macro_rules! Depcrate_argsSubscription {
() => {
// Module: crate::args
// Provides: {"Subscription"}
// Dependencies: {}
# [derive (FromMeta , Default)] # [darling (default)] pub struct Subscription { pub internal : bool , pub name : Option < String > , # [darling (default)] pub name_type : bool , pub rename_fields : Option < RenameRule > , pub rename_args : Option < RenameRule > , pub use_type_description : bool , pub extends : bool , pub visible : Option < Visible > , # [darling (default)] pub guard : Option < Expr > , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , }
};
}
