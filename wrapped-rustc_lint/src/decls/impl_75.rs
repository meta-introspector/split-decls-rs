macro_rules! deps {
    () => {
        BuiltinDoubleNegationsAddParens!();
        BuiltinDoubleNegations!();
        EarlyContext!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl EarlyLintPass for DoubleNegations { # [inline] fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & ast :: Expr) { if let ExprKind :: Unary (UnOp :: Neg , ref inner) = expr . kind && let ExprKind :: Unary (UnOp :: Neg , ref inner2) = inner . kind && ! matches ! (inner2 . kind , ExprKind :: Unary (UnOp :: Neg , _)) && expr . span . eq_ctxt (inner . span) { cx . emit_span_lint (DOUBLE_NEGATIONS , expr . span , BuiltinDoubleNegations { add_parens : BuiltinDoubleNegationsAddParens { start_span : inner . span . shrink_to_lo () , end_span : inner . span . shrink_to_hi () , } , } ,) ; } } }
    };
}

impl_75!()