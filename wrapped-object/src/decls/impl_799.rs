macro_rules! deps {
    () => {
        SectionTable!();
        FileHeader!();
    };
}

macro_rules! impl_799 {
    () => {
        deps!();
        impl < 'data , Xcoff > Default for SectionTable < 'data , Xcoff > where Xcoff : FileHeader , { fn default () -> Self { Self { sections : & [] } } }
    };
}

impl_799!();