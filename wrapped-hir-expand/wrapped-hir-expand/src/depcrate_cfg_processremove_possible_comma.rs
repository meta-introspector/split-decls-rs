// Generated macro for remove_possible_comma (function)
macro_rules! Depcrate_cfg_processremove_possible_comma {
() => {
// Module: crate::cfg_process
// Provides: {"remove_possible_comma"}
// Dependencies: {}
# [doc = " Removes a possible comma after the [AstNode]"] fn remove_possible_comma (item : & impl AstNode , res : & mut FxHashSet < SyntaxElement >) { if let Some (comma) = item . syntax () . next_sibling_or_token () . filter (| it | it . kind () == T ! [,]) { res . insert (comma) ; } }
};
}
