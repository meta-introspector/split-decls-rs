macro_rules! deps {
    () => {
        SectionId!();
    };
}

macro_rules! impl_1121 {
    () => {
        deps!();
        impl IdPrivate for SectionId { fn new (id : usize) -> Self { SectionId (id) } }
    };
}

impl_1121!();