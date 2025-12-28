macro_rules! deps {
    () => {
        ResolverError!();
        FluentError!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl From < ResolverError > for FluentError { fn from (error : ResolverError) -> Self { Self :: ResolverError (error) } }
    };
}

impl_30!();