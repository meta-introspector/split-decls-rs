// Generated macro for is_type_diagnostic_item (function)
macro_rules! Depcrate_tyis_type_diagnostic_item {
() => {
// Module: crate::ty
// Provides: {"is_type_diagnostic_item"}
// Dependencies: {}
# [doc = " Checks if the type is equal to a diagnostic item. To check if a type implements a"] # [doc = " trait marked with a diagnostic item use [`implements_trait`]."] # [doc = ""] # [doc = " For a further exploitation what diagnostic items are see [diagnostic items] in"] # [doc = " rustc-dev-guide."] # [doc = ""] # [doc = " ---"] # [doc = ""] # [doc = " If you change the signature, remember to update the internal lint `MatchTypeOnDiagItem`"] # [doc = ""] # [doc = " [Diagnostic Items]: https://rustc-dev-guide.rust-lang.org/diagnostics/diagnostic-items.html"] pub fn is_type_diagnostic_item (cx : & LateContext < '_ > , ty : Ty < '_ > , diag_item : Symbol) -> bool { match ty . kind () { ty :: Adt (adt , _) => cx . tcx . is_diagnostic_item (diag_item , adt . did ()) , _ => false , } }
};
}
