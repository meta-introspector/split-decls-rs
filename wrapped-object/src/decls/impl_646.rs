macro_rules! deps {
    () => {
        ReadRef!();
        SectionIndex!();
        PeComdatSectionIterator!();
        ImageNtHeaders!();
        Item!();
    };
}

macro_rules! impl_646 {
    () => {
        deps!();
        impl < 'data , 'file , Pe , R > Iterator for PeComdatSectionIterator < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { type Item = SectionIndex ; fn next (& mut self) -> Option < Self :: Item > { None } }
    };
}

impl_646!()