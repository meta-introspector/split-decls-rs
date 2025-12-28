macro_rules! deps {
    () => {
        StackVec!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl Eq for StackVec { }
    };
}

impl_133!();