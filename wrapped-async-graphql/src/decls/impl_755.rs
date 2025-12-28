macro_rules! deps {
    () => {
        EmptyMutation!();
        ObjectType!();
    };
}

macro_rules! impl_755 {
    () => {
        deps!();
        impl ObjectType for EmptyMutation { }
    };
}

impl_755!();