// Generated macro for eq_angle_arg (function)
macro_rules! Depcrate_ast_utilseq_angle_arg {
() => {
// Module: crate::ast_utils
// Provides: {"eq_angle_arg"}
// Dependencies: {}
pub fn eq_angle_arg (l : & AngleBracketedArg , r : & AngleBracketedArg) -> bool { match (l , r) { (AngleBracketedArg :: Arg (l) , AngleBracketedArg :: Arg (r)) => eq_generic_arg (l , r) , (AngleBracketedArg :: Constraint (l) , AngleBracketedArg :: Constraint (r)) => eq_assoc_item_constraint (l , r) , _ => false , } }
};
}
