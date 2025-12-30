// Generated macro for check_remainder (function)
macro_rules! Depcrate_operators_identity_opcheck_remainder {
() => {
// Module: crate::operators::identity_op
// Provides: {"check_remainder"}
// Dependencies: {}
fn check_remainder (cx : & LateContext < '_ > , left : & Expr < '_ > , right : & Expr < '_ > , span : Span , arg : Span) { let ecx = ConstEvalCtxt :: new (cx) ; let ctxt = span . ctxt () ; if match (ecx . eval_full_int (left , ctxt) , ecx . eval_full_int (right , ctxt)) { (Some (FullInt :: S (lv)) , Some (FullInt :: S (rv))) => lv . abs () < rv . abs () , (Some (FullInt :: U (lv)) , Some (FullInt :: U (rv))) => lv < rv , _ => return , } { span_ineffective_operation (cx , span , arg , Parens :: Unneeded , false) ; } }
};
}
