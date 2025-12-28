macro_rules! deps {
    () => {
        ObjectType!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < T : ObjectType + ? Sized > ObjectType for Arc < T > { }
    };
}

impl_9!();