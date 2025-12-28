macro_rules! deps {
    () => {
        ResolverError!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < T > From < & InlineExpression < T > > for ResolverError where T : ToString , { fn from (exp : & InlineExpression < T >) -> Self { Self :: Reference (exp . into ()) } }
    };
}

impl_46!()