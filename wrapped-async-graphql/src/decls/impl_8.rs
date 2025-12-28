macro_rules! deps {
    () => {
        ObjectType!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < T : ObjectType + ? Sized > ObjectType for Box < T > { }
    };
}

impl_8!();