macro_rules! deps {
    () => {
        MachHeader!();
        SymbolIndex!();
        SectionKind!();
        Section!();
        SectionIndex!();
        SymbolScope!();
        ObjectSymbol!();
        Dynamic!();
        MachO!();
        SymbolKind!();
        ReadRef!();
        Result!();
        MachOSymbol!();
        SymbolSection!();
        SymbolFlags!();
    };
}

macro_rules! impl_616 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > ObjectSymbol < 'data > for MachOSymbol < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { # [inline] fn index (& self) -> SymbolIndex { self . index } fn name_bytes (& self) -> Result < & 'data [u8] > { self . nlist . name (self . file . endian , self . file . symbols . strings) } fn name (& self) -> Result < & 'data str > { let name = self . name_bytes () ? ; str :: from_utf8 (name) . ok () . read_error ("Non UTF-8 Mach-O symbol name") } # [inline] fn address (& self) -> u64 { self . nlist . n_value (self . file . endian) . into () } # [inline] fn size (& self) -> u64 { 0 } fn kind (& self) -> SymbolKind { self . section () . index () . and_then (| index | self . file . section_internal (index) . ok ()) . map (| section | match section . kind { SectionKind :: Text => SymbolKind :: Text , SectionKind :: Data | SectionKind :: ReadOnlyData | SectionKind :: ReadOnlyString | SectionKind :: UninitializedData | SectionKind :: Common => SymbolKind :: Data , SectionKind :: Tls | SectionKind :: UninitializedTls | SectionKind :: TlsVariables => { SymbolKind :: Tls } _ => SymbolKind :: Unknown , }) . unwrap_or (SymbolKind :: Unknown) } fn section (& self) -> SymbolSection { match self . nlist . n_type () & macho :: N_TYPE { macho :: N_UNDF => SymbolSection :: Undefined , macho :: N_ABS => SymbolSection :: Absolute , macho :: N_SECT => { let n_sect = self . nlist . n_sect () ; if n_sect != 0 { SymbolSection :: Section (SectionIndex (n_sect as usize)) } else { SymbolSection :: Unknown } } _ => SymbolSection :: Unknown , } } # [inline] fn is_undefined (& self) -> bool { self . nlist . n_type () & macho :: N_TYPE == macho :: N_UNDF } # [inline] fn is_definition (& self) -> bool { self . nlist . is_definition () } # [inline] fn is_common (& self) -> bool { false } # [inline] fn is_weak (& self) -> bool { self . nlist . n_desc (self . file . endian) & (macho :: N_WEAK_REF | macho :: N_WEAK_DEF) != 0 } fn scope (& self) -> SymbolScope { let n_type = self . nlist . n_type () ; if n_type & macho :: N_TYPE == macho :: N_UNDF { SymbolScope :: Unknown } else if n_type & macho :: N_EXT == 0 { SymbolScope :: Compilation } else if n_type & macho :: N_PEXT != 0 { SymbolScope :: Linkage } else { SymbolScope :: Dynamic } } # [inline] fn is_global (& self) -> bool { self . scope () != SymbolScope :: Compilation } # [inline] fn is_local (& self) -> bool { self . scope () == SymbolScope :: Compilation } # [inline] fn flags (& self) -> SymbolFlags < SectionIndex , SymbolIndex > { let n_desc = self . nlist . n_desc (self . file . endian) ; SymbolFlags :: MachO { n_desc } } }
    };
}

impl_616!();