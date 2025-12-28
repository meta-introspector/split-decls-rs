macro_rules! deps {
    () => {
        PatId!();
        ExpressionStore!();
        Pat!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl Index < PatId > for ExpressionStore { type Output = Pat ; # [inline] fn index (& self , pat : PatId) -> & Pat { & self . assert_expr_only () . pats [pat] } }
    };
}

impl_293!()