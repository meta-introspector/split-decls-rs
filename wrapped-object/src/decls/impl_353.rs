macro_rules! deps {
    () => {
        Item!();
        Relocation!();
        Rel!();
        ElfRelocationIterator!();
        ReadRef!();
        Crel!();
        CrelIterator!();
        Rela!();
        FileHeader!();
        ElfDynamicRelocationIterator!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > Iterator for ElfDynamicRelocationIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { type Item = (u64 , Relocation) ; fn next (& mut self) -> Option < Self :: Item > { let endian = self . file . endian ; loop { if let Some (ref mut relocations) = self . relocations { if let Some (reloc) = relocations . next () { let relocation = parse_relocation (self . file . header , endian , reloc , relocations . is_rel ()) ; return Some ((reloc . r_offset , relocation)) ; } self . relocations = None ; } let section = self . file . sections . section (self . section_index) . ok () ? ; self . section_index . 0 += 1 ; if section . link (endian) != self . file . dynamic_symbols . section () { continue ; } match section . sh_type (endian) { elf :: SHT_REL => { if let Ok (relocations) = section . data_as_array (endian , self . file . data) { self . relocations = Some (ElfRelocationIterator :: Rel (relocations . iter () , endian)) ; } } elf :: SHT_RELA => { if let Ok (relocations) = section . data_as_array (endian , self . file . data) { self . relocations = Some (ElfRelocationIterator :: Rela (relocations . iter () , endian , self . file . header . is_mips64el (endian) ,)) ; } } elf :: SHT_CREL => { if let Ok (data) = section . data (endian , self . file . data) { if let Ok (relocations) = CrelIterator :: new (data) { self . relocations = Some (ElfRelocationIterator :: Crel (relocations)) ; } } } _ => { } } } } }
    };
}

impl_353!()