macro_rules! deps {
    () => {
        SectionIndex!();
        ElfSectionIterator!();
        FileHeader!();
        ReadRef!();
        ElfSection!();
        Item!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > Iterator for ElfSectionIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { type Item = ElfSection < 'data , 'file , Elf , R > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (index , section) | ElfSection { index : SectionIndex (index) , file : self . file , section , }) } }
    };
}

impl_310!()