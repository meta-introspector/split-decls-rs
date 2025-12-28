macro_rules! deps {
    () => {
        PeSection!();
        ReadRef!();
        ImageNtHeaders!();
        PeSectionIterator!();
        SectionIndex!();
        Item!();
    };
}

macro_rules! impl_669 {
    () => {
        deps!();
        impl < 'data , 'file , Pe , R > Iterator for PeSectionIterator < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { type Item = PeSection < 'data , 'file , Pe , R > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (index , section) | PeSection { file : self . file , index : SectionIndex (index + 1) , section , }) } }
    };
}

impl_669!()