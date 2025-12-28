macro_rules! deps {
    () => {
        Binding!();
        BindingId!();
        ExpressionStore!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl Index < BindingId > for ExpressionStore { type Output = Binding ; # [inline] fn index (& self , b : BindingId) -> & Binding { & self . assert_expr_only () . bindings [b] } }
    };
}

impl_295!();