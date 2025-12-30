// Generated macro for get_type_diagnostic_name (function)
macro_rules! Depcrate_tyget_type_diagnostic_name {
() => {
// Module: crate::ty
// Provides: {"get_type_diagnostic_name"}
// Dependencies: {}
# [doc = " Get the diagnostic name of a type, e.g. `sym::HashMap`. To check if a type"] # [doc = " implements a trait marked with a diagnostic item use [`implements_trait`]."] # [doc = ""] # [doc = " For a further exploitation what diagnostic items are see [diagnostic items] in"] # [doc = " rustc-dev-guide."] # [doc = ""] # [doc = " [Diagnostic Items]: https://rustc-dev-guide.rust-lang.org/diagnostics/diagnostic-items.html"] pub fn get_type_diagnostic_name (cx : & LateContext < '_ > , ty : Ty < '_ >) -> Option < Symbol > { match ty . kind () { ty :: Adt (adt , _) => cx . tcx . get_diagnostic_name (adt . did ()) , _ => None , } }
};
}
