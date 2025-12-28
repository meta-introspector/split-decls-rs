macro_rules! deps {
    () => {
        ExpressionStore!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl Index < LifetimeRefId > for ExpressionStore { type Output = LifetimeRef ; # [inline] fn index (& self , b : LifetimeRefId) -> & LifetimeRef { & self . lifetimes [b] } }
    };
}

impl_297!();