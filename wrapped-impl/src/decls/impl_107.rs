macro_rules! deps {
    () => {
        MemberUnraw!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl Eq for MemberUnraw { }
    };
}

impl_107!()