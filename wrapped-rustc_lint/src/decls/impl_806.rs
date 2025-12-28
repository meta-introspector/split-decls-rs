macro_rules! deps {
    () => {
        TypeLimits!();
    };
}

macro_rules! impl_806 {
    () => {
        deps!();
        impl TypeLimits { pub (crate) fn new () -> TypeLimits { TypeLimits { negated_expr_id : None , negated_expr_span : None } } }
    };
}

impl_806!();