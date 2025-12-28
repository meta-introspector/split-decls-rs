macro_rules! deps {
    () => {
        Label!();
        ExpressionStore!();
        LabelId!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl Index < LabelId > for ExpressionStore { type Output = Label ; # [inline] fn index (& self , label : LabelId) -> & Label { & self . assert_expr_only () . labels [label] } }
    };
}

impl_294!();