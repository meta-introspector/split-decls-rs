macro_rules! deps {
    () => {
        ExprId!();
        ExpressionStore!();
        Expr!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl Index < ExprId > for ExpressionStore { type Output = Expr ; # [inline] fn index (& self , expr : ExprId) -> & Expr { & self . assert_expr_only () . exprs [expr] } }
    };
}

impl_292!();