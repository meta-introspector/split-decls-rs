// Generated macro for check_ln1p (function)
macro_rules! Depcrate_floating_point_arithmeticcheck_ln1p {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"check_ln1p"}
// Dependencies: {}
fn check_ln1p (cx : & LateContext < '_ > , expr : & Expr < '_ > , receiver : & Expr < '_ >) { if let ExprKind :: Binary (Spanned { node : BinOpKind :: Add , .. } , lhs , rhs ,) = receiver . kind { let ecx = ConstEvalCtxt :: new (cx) ; let recv = match (ecx . eval (lhs) , ecx . eval (rhs)) { (Some (value) , _) if F32 (1.0) == value || F64 (1.0) == value => rhs , (_ , Some (value)) if F32 (1.0) == value || F64 (1.0) == value => lhs , _ => return , } ; span_lint_and_sugg (cx , IMPRECISE_FLOPS , expr . span , "ln(1 + x) can be computed more accurately" , "consider using" , format ! ("{}.ln_1p()" , prepare_receiver_sugg (cx , recv)) , Applicability :: MachineApplicable ,) ; } }
};
}
