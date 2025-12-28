macro_rules! deps {
    () => {
        ObjectType!();
        EmptyMutation!();
    };
}

macro_rules! impl_755 {
    () => {
        deps!();
        impl ObjectType for EmptyMutation { }
    };
}

impl_755!()