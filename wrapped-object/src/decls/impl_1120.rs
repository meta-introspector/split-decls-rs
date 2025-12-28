macro_rules! deps {
    () => {
        SectionId!();
        Id!();
    };
}

macro_rules! impl_1120 {
    () => {
        deps!();
        impl Id for SectionId { fn index (& self) -> usize { self . 0 } }
    };
}

impl_1120!();