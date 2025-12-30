// Generated macro for ContainsExpr (struct)
macro_rules! Depcrate_entryContainsExpr {
() => {
// Module: crate::entry
// Provides: {"ContainsExpr"}
// Dependencies: {}
# [doc = " Details on an expression checking whether a map contains a key."] # [doc = ""] # [doc = " For instance, with the following:"] # [doc = " ```ignore"] # [doc = " !!!self.the_map.contains_key(\"the_key\")"] # [doc = " ```"] # [doc = ""] # [doc = " - `negated` will be set to `true` (the 3 `!` negate the condition)"] # [doc = " - `map` will be the `self.the_map` expression"] # [doc = " - `key` will be the `\"the_key\"` expression"] struct ContainsExpr < 'tcx > { # [doc = " Whether the check for `contains_key` was negated."] negated : bool , # [doc = " The map on which the check is performed."] map : & 'tcx Expr < 'tcx > , # [doc = " The key that is checked to be contained."] key : & 'tcx Expr < 'tcx > , # [doc = " The context of the whole condition expression."] call_ctxt : SyntaxContext , }
};
}
