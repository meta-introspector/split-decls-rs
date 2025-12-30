// Generated macro for ExprUseNode (enum)
macro_rules! DepcrateExprUseNode {
() => {
// Module: crate
// Provides: {"ExprUseNode"}
// Dependencies: {}
# [doc = " The node which consumes a value."] pub enum ExprUseNode < 'tcx > { # [doc = " Assignment to, or initializer for, a local"] LetStmt (& 'tcx LetStmt < 'tcx >) , # [doc = " Initializer for a const or static item."] ConstStatic (OwnerId) , # [doc = " Implicit or explicit return from a function."] Return (OwnerId) , # [doc = " Initialization of a struct field."] Field (& 'tcx ExprField < 'tcx >) , # [doc = " An argument to a function."] FnArg (& 'tcx Expr < 'tcx > , usize) , # [doc = " An argument to a method."] MethodArg (HirId , Option < & 'tcx GenericArgs < 'tcx > > , usize) , # [doc = " The callee of a function call."] Callee , # [doc = " Access of a field."] FieldAccess (Ident) , # [doc = " Borrow expression."] AddrOf (ast :: BorrowKind , Mutability) , Other , }
};
}
