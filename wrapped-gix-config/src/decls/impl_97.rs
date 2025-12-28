macro_rules! deps {
    () => {
        SectionId!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl Default for SectionId { fn default () -> Self { SectionId (usize :: MAX) } }
    };
}

impl_97!()