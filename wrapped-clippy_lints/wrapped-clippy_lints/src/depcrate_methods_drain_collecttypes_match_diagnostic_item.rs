// Generated macro for types_match_diagnostic_item (function)
macro_rules! Depcrate_methods_drain_collecttypes_match_diagnostic_item {
() => {
// Module: crate::methods::drain_collect
// Provides: {"types_match_diagnostic_item"}
// Dependencies: {}
# [doc = " Checks if both types match the given diagnostic item, e.g.:"] # [doc = ""] # [doc = " `vec![1,2].drain(..).collect::<Vec<_>>()`"] # [doc = "  ^^^^^^^^^                     ^^^^^^   true"] # [doc = " `vec![1,2].drain(..).collect::<HashSet<_>>()`"] # [doc = "  ^^^^^^^^^                     ^^^^^^^^^^  false"] fn types_match_diagnostic_item (cx : & LateContext < '_ > , expr : Ty < '_ > , recv : Ty < '_ > , sym : Symbol) -> bool { if let Some (expr_adt) = expr . ty_adt_def () && let Some (recv_adt) = recv . ty_adt_def () { cx . tcx . is_diagnostic_item (sym , expr_adt . did ()) && cx . tcx . is_diagnostic_item (sym , recv_adt . did ()) } else { false } }
};
}
