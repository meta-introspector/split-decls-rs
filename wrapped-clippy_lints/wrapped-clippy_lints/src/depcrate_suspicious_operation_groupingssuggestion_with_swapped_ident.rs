// Generated macro for suggestion_with_swapped_ident (function)
macro_rules! Depcrate_suspicious_operation_groupingssuggestion_with_swapped_ident {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"suggestion_with_swapped_ident"}
// Dependencies: {}
fn suggestion_with_swapped_ident (cx : & EarlyContext < '_ > , expr : & Expr , location : IdentLocation , new_ident : Ident , applicability : & mut Applicability ,) -> Option < String > { get_ident (expr , location) . and_then (| current_ident | { if eq_id (current_ident , new_ident) { return None ; } Some (format ! ("{}{new_ident}{}" , snippet_with_applicability (cx , expr . span . with_hi (current_ident . span . lo ()) , ".." , applicability) , snippet_with_applicability (cx , expr . span . with_lo (current_ident . span . hi ()) , ".." , applicability) ,)) }) }
};
}
