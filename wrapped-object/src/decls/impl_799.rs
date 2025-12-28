macro_rules! deps {
    () => {
        FileHeader!();
        SectionTable!();
    };
}

macro_rules! impl_799 {
    () => {
        deps!();
        impl < 'data , Xcoff > Default for SectionTable < 'data , Xcoff > where Xcoff : FileHeader , { fn default () -> Self { Self { sections : & [] } } }
    };
}

impl_799!()