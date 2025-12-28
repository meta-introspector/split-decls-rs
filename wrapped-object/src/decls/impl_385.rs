macro_rules! deps {
    () => {
        ElfComdatIterator!();
        ElfComdat!();
        ReadRef!();
        FileHeader!();
        Item!();
    };
}

macro_rules! impl_385 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > Iterator for ElfComdatIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { type Item = ElfComdat < 'data , 'file , Elf , R > ; fn next (& mut self) -> Option < Self :: Item > { for (_index , section) in self . iter . by_ref () { if let Some (comdat) = ElfComdat :: parse (self . file , section) { return Some (comdat) ; } } None } }
    };
}

impl_385!();