// Generated macro for binop_to_string (function)
macro_rules! Depcrate_suggbinop_to_string {
() => {
// Module: crate::sugg
// Provides: {"binop_to_string"}
// Dependencies: {}
# [doc = " Generates a string from the operator and both sides."] fn binop_to_string (op : AssocOp , lhs : & str , rhs : & str) -> String { match op { AssocOp :: Binary (op) => format ! ("{lhs} {} {rhs}" , op . as_str ()) , AssocOp :: Assign => format ! ("{lhs} = {rhs}") , AssocOp :: AssignOp (op) => format ! ("{lhs} {} {rhs}" , op . as_str ()) , AssocOp :: Cast => format ! ("{lhs} as {rhs}") , AssocOp :: Range (limits) => format ! ("{lhs}{}{rhs}" , limits . as_str ()) , } }
};
}
