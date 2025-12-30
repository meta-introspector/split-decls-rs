// Generated macro for tests (module)
macro_rules! Depcrate_optimizer_restorertests {
() => {
// Module: crate::optimizer::restorer
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: optimizer :: OptimizedExpr :: * ; # [test] fn restore_no_stack_children () { let rules = vec ! [OptimizedRule { name : "rule" . to_owned () , ty : RuleType :: Normal , expr : box_tree ! (Opt (Str ("a" . to_string ()))) , }] ; assert_eq ! (restore_on_err (rules [0] . clone () , & to_optimized_hash_map (& rules)) , rules [0] . clone ()) ; } # [test] fn restore_with_child_stack_ops () { let rules = vec ! [OptimizedRule { name : "rule" . to_owned () , ty : RuleType :: Normal , expr : box_tree ! (Rep (Push (Str ("a" . to_string ())))) , }] ; let restored = OptimizedRule { name : "rule" . to_owned () , ty : RuleType :: Normal , expr : box_tree ! (Rep (RestoreOnErr (Push (Str ("a" . to_string ()))))) , } ; assert_eq ! (restore_on_err (rules [0] . clone () , & to_optimized_hash_map (& rules)) , restored) ; } # [test] fn restore_choice_branch_with_and_branch_without () { let rules = vec ! [OptimizedRule { name : "rule" . to_owned () , ty : RuleType :: Normal , expr : box_tree ! (Choice (Push (Str ("a" . to_string ())) , Str ("a" . to_string ()))) , }] ; let restored = OptimizedRule { name : "rule" . to_owned () , ty : RuleType :: Normal , expr : box_tree ! (Choice (RestoreOnErr (Push (Str ("a" . to_string ()))) , Str ("a" . to_string ()))) , } ; assert_eq ! (restore_on_err (rules [0] . clone () , & to_optimized_hash_map (& rules)) , restored) ; } }
};
}
