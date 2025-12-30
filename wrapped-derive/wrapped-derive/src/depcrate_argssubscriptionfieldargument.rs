// Generated macro for SubscriptionFieldArgument (struct)
macro_rules! Depcrate_argsSubscriptionFieldArgument {
() => {
// Module: crate::args
// Provides: {"SubscriptionFieldArgument"}
// Dependencies: {}
# [derive (FromMeta , Default)] # [darling (default)] pub struct SubscriptionFieldArgument { pub name : Option < String > , pub desc : Option < String > , pub default : Option < DefaultValue > , pub default_with : Option < LitStr > , pub validator : Option < Validators > , # [darling (default)] pub process_with : Option < Expr > , pub visible : Option < Visible > , pub secret : bool , pub deprecation : Deprecation , }
};
}
