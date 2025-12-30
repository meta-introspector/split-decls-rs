// Generated macro for SubscriptionField (struct)
macro_rules! Depcrate_argsSubscriptionField {
() => {
// Module: crate::args
// Provides: {"SubscriptionField"}
// Dependencies: {}
# [derive (FromMeta , Default)] # [darling (default)] pub struct SubscriptionField { pub skip : bool , pub name : Option < String > , pub deprecation : Deprecation , pub guard : Option < Expr > , pub visible : Option < Visible > , pub complexity : Option < Expr > , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , }
};
}
