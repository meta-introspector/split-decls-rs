macro_rules! deps {
    () => {
        QueryRoot!();
        ObjectType!();
    };
}

macro_rules! impl_806 {
    () => {
        deps!();
        impl < T : ObjectType > ObjectType for QueryRoot < T > { }
    };
}

impl_806!()