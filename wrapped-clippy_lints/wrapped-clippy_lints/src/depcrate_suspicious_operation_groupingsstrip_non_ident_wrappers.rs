// Generated macro for strip_non_ident_wrappers (function)
macro_rules! Depcrate_suspicious_operation_groupingsstrip_non_ident_wrappers {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"strip_non_ident_wrappers"}
// Dependencies: {}
fn strip_non_ident_wrappers (expr : & Expr) -> & Expr { let mut output = expr ; loop { output = match & output . kind { ExprKind :: Paren (inner) | ExprKind :: Unary (_ , inner) => inner , _ => { return output ; } , } ; } }
};
}
