macro_rules! deps {
    () => {
        ReadRef!();
        Item!();
        ElfComdatSectionIterator!();
        FileHeader!();
        SectionIndex!();
    };
}

macro_rules! impl_395 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > Iterator for ElfComdatSectionIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { type Item = SectionIndex ; fn next (& mut self) -> Option < Self :: Item > { let index = self . sections . next () ? ; Some (SectionIndex (index . get (self . file . endian) as usize)) } }
    };
}

impl_395!();