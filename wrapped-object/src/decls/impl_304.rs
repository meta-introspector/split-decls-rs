macro_rules! deps {
    () => {
        ReadRef!();
        StringTable!();
        FileHeader!();
        SectionTable!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader , R : ReadRef < 'data > > Default for SectionTable < 'data , Elf , R > { fn default () -> Self { SectionTable { sections : & [] , strings : StringTable :: default () , } } }
    };
}

impl_304!();