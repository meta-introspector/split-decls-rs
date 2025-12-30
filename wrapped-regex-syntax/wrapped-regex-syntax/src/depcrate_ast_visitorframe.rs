// Generated macro for Frame (enum)
macro_rules! Depcrate_ast_visitorFrame {
() => {
// Module: crate::ast::visitor
// Provides: {"Frame"}
// Dependencies: {}
# [doc = " Represents a single stack frame while performing structural induction over"] # [doc = " an `Ast`."] enum Frame < 'a > { # [doc = " A stack frame allocated just before descending into a repetition"] # [doc = " operator's child node."] Repetition (& 'a ast :: Repetition) , # [doc = " A stack frame allocated just before descending into a group's child"] # [doc = " node."] Group (& 'a ast :: Group) , # [doc = " The stack frame used while visiting every child node of a concatenation"] # [doc = " of expressions."] Concat { # [doc = " The child node we are currently visiting."] head : & 'a Ast , # [doc = " The remaining child nodes to visit (which may be empty)."] tail : & 'a [Ast] , } , # [doc = " The stack frame used while visiting every child node of an alternation"] # [doc = " of expressions."] Alternation { # [doc = " The child node we are currently visiting."] head : & 'a Ast , # [doc = " The remaining child nodes to visit (which may be empty)."] tail : & 'a [Ast] , } , }
};
}
