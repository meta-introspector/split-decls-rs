macro_rules! deps {
    () => {
        ObjectType!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < T : ObjectType + ? Sized > ObjectType for & T { }
    };
}

impl_7!()