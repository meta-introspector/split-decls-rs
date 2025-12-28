macro_rules! deps {
    () => {
        SymbolKind!();
        ObjectSymbol!();
        FileHeader!();
        Section!();
        SymbolScope!();
        Dynamic!();
        SectionIndex!();
        Result!();
        ReadRef!();
        ElfSymbol!();
        SymbolFlags!();
        SymbolSection!();
        File!();
        SymbolIndex!();
    };
}

macro_rules! impl_340 {
    () => {
        deps!();
        impl < 'data , 'file , Elf : FileHeader , R : ReadRef < 'data > > ObjectSymbol < 'data > for ElfSymbol < 'data , 'file , Elf , R > { # [inline] fn index (& self) -> SymbolIndex { self . index } fn name_bytes (& self) -> read :: Result < & 'data [u8] > { self . symbol . name (self . endian , self . symbols . strings ()) } fn name (& self) -> read :: Result < & 'data str > { let name = self . name_bytes () ? ; str :: from_utf8 (name) . ok () . read_error ("Non UTF-8 ELF symbol name") } # [inline] fn address (& self) -> u64 { self . symbol . st_value (self . endian) . into () } # [inline] fn size (& self) -> u64 { self . symbol . st_size (self . endian) . into () } fn kind (& self) -> SymbolKind { match self . symbol . st_type () { elf :: STT_NOTYPE => SymbolKind :: Unknown , elf :: STT_OBJECT | elf :: STT_COMMON => SymbolKind :: Data , elf :: STT_FUNC | elf :: STT_GNU_IFUNC => SymbolKind :: Text , elf :: STT_SECTION => SymbolKind :: Section , elf :: STT_FILE => SymbolKind :: File , elf :: STT_TLS => SymbolKind :: Tls , _ => SymbolKind :: Unknown , } } fn section (& self) -> SymbolSection { match self . symbol . st_shndx (self . endian) { elf :: SHN_UNDEF => SymbolSection :: Undefined , elf :: SHN_ABS => { if self . symbol . st_type () == elf :: STT_FILE { SymbolSection :: None } else { SymbolSection :: Absolute } } elf :: SHN_COMMON => SymbolSection :: Common , elf :: SHN_XINDEX => match self . symbols . shndx (self . endian , self . index) { Some (0) => SymbolSection :: None , Some (index) => SymbolSection :: Section (SectionIndex (index as usize)) , None => SymbolSection :: Unknown , } , index if index < elf :: SHN_LORESERVE => { SymbolSection :: Section (SectionIndex (index as usize)) } _ => SymbolSection :: Unknown , } } # [inline] fn is_undefined (& self) -> bool { self . symbol . is_undefined (self . endian) } # [inline] fn is_definition (& self) -> bool { self . symbol . is_definition (self . endian) } # [inline] fn is_common (& self) -> bool { self . symbol . is_common (self . endian) } # [inline] fn is_weak (& self) -> bool { self . symbol . is_weak () } fn scope (& self) -> SymbolScope { if self . symbol . st_shndx (self . endian) == elf :: SHN_UNDEF { SymbolScope :: Unknown } else { match self . symbol . st_bind () { elf :: STB_LOCAL => SymbolScope :: Compilation , elf :: STB_GLOBAL | elf :: STB_WEAK => { if self . symbol . st_visibility () == elf :: STV_HIDDEN { SymbolScope :: Linkage } else { SymbolScope :: Dynamic } } _ => SymbolScope :: Unknown , } } } # [inline] fn is_global (& self) -> bool { ! self . symbol . is_local () } # [inline] fn is_local (& self) -> bool { self . symbol . is_local () } # [inline] fn flags (& self) -> SymbolFlags < SectionIndex , SymbolIndex > { SymbolFlags :: Elf { st_info : self . symbol . st_info () , st_other : self . symbol . st_other () , } } }
    };
}

impl_340!()