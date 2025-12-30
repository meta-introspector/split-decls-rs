// Generated macro for ClassFrame (enum)
macro_rules! Depcrate_ast_visitorClassFrame {
() => {
// Module: crate::ast::visitor
// Provides: {"ClassFrame"}
// Dependencies: {}
# [doc = " Represents a single stack frame while performing structural induction over"] # [doc = " a character class."] enum ClassFrame < 'a > { # [doc = " The stack frame used while visiting every child node of a union of"] # [doc = " character class items."] Union { # [doc = " The child node we are currently visiting."] head : & 'a ast :: ClassSetItem , # [doc = " The remaining child nodes to visit (which may be empty)."] tail : & 'a [ast :: ClassSetItem] , } , # [doc = " The stack frame used while a binary class operation."] Binary { op : & 'a ast :: ClassSetBinaryOp } , # [doc = " A stack frame allocated just before descending into a binary operator's"] # [doc = " left hand child node."] BinaryLHS { op : & 'a ast :: ClassSetBinaryOp , lhs : & 'a ast :: ClassSet , rhs : & 'a ast :: ClassSet , } , # [doc = " A stack frame allocated just before descending into a binary operator's"] # [doc = " right hand child node."] BinaryRHS { op : & 'a ast :: ClassSetBinaryOp , rhs : & 'a ast :: ClassSet } , }
};
}
