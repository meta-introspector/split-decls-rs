macro_rules! deps {
    () => {
        ComdatKind!();
        ReadRef!();
        ElfComdatSectionIterator!();
        Result!();
        FileHeader!();
        ObjectComdat!();
        SectionIterator!();
        ElfComdat!();
        SymbolIndex!();
    };
}

macro_rules! impl_391 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > ObjectComdat < 'data > for ElfComdat < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { type SectionIterator = ElfComdatSectionIterator < 'data , 'file , Elf , R > ; # [inline] fn kind (& self) -> ComdatKind { ComdatKind :: Any } # [inline] fn symbol (& self) -> SymbolIndex { SymbolIndex (self . section . sh_info (self . file . endian) as usize) } fn name_bytes (& self) -> read :: Result < & 'data [u8] > { let index = self . symbol () ; let symbol = self . file . symbols . symbol (index) ? ; symbol . name (self . file . endian , self . file . symbols . strings ()) } fn name (& self) -> read :: Result < & 'data str > { let name = self . name_bytes () ? ; str :: from_utf8 (name) . ok () . read_error ("Non UTF-8 ELF COMDAT name") } fn sections (& self) -> Self :: SectionIterator { ElfComdatSectionIterator { file : self . file , sections : self . sections . iter () , } } }
    };
}

impl_391!()