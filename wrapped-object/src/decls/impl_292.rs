macro_rules! deps {
    () => {
        Item!();
        FileHeader!();
        ElfSegment!();
        ElfSegmentIterator!();
        ReadRef!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > Iterator for ElfSegmentIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { type Item = ElfSegment < 'data , 'file , Elf , R > ; fn next (& mut self) -> Option < Self :: Item > { for segment in self . iter . by_ref () { if segment . p_type (self . file . endian) == elf :: PT_LOAD { return Some (ElfSegment { file : self . file , segment , }) ; } } None } }
    };
}

impl_292!();