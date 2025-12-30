// Generated macro for chained_binops_helper (function)
macro_rules! Depcrate_suspicious_operation_groupingschained_binops_helper {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"chained_binops_helper"}
// Dependencies: {}
fn chained_binops_helper < 'expr > (left_outer : & 'expr Expr , right_outer : & 'expr Expr) -> Option < Vec < BinaryOp < 'expr > > > { match (& left_outer . kind , & right_outer . kind) { (ExprKind :: Paren (left_e) | ExprKind :: Unary (_ , left_e) , ExprKind :: Paren (right_e) | ExprKind :: Unary (_ , right_e) ,) => chained_binops_helper (left_e , right_e) , (ExprKind :: Paren (left_e) | ExprKind :: Unary (_ , left_e) , _) => chained_binops_helper (left_e , right_outer) , (_ , ExprKind :: Paren (right_e) | ExprKind :: Unary (_ , right_e)) => chained_binops_helper (left_outer , right_e) , (ExprKind :: Binary (Spanned { node : left_op , .. } , left_left , left_right) , ExprKind :: Binary (Spanned { node : right_op , .. } , right_left , right_right) ,) => match (chained_binops_helper (left_left , left_right) , chained_binops_helper (right_left , right_right) ,) { (Some (mut left_ops) , Some (right_ops)) => { left_ops . reserve (right_ops . len ()) ; for op in right_ops { left_ops . push (op) ; } Some (left_ops) } , (Some (mut left_ops) , _) => { left_ops . push (BinaryOp :: new (* right_op , right_outer . span , (right_left , right_right))) ; Some (left_ops) } , (_ , Some (mut right_ops)) => { right_ops . insert (0 , BinaryOp :: new (* left_op , left_outer . span , (left_left , left_right))) ; Some (right_ops) } , (None , None) => Some (vec ! [BinaryOp :: new (* left_op , left_outer . span , (left_left , left_right)) , BinaryOp :: new (* right_op , right_outer . span , (right_left , right_right)) ,]) , } , _ => None , } }
};
}
