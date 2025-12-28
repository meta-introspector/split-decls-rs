macro_rules! deps {
    () => {
        Relr!();
        NoteHeader!();
        ProgramHeader!();
        Rel!();
        FileHeader!();
        Dyn!();
        SectionHeader64!();
        Relr64!();
        Ident!();
        CompressionHeader!();
        Rela!();
        Sym!();
        Dyn64!();
        Rel64!();
        FileHeader64!();
        ProgramHeader64!();
        Rela64!();
        Endian!();
        NoteHeader32!();
        SectionHeader!();
        CompressionHeader64!();
        Sym64!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > FileHeader for elf :: FileHeader64 < Endian > { type Word = u64 ; type Sword = i64 ; type Endian = Endian ; type ProgramHeader = elf :: ProgramHeader64 < Endian > ; type SectionHeader = elf :: SectionHeader64 < Endian > ; type CompressionHeader = elf :: CompressionHeader64 < Endian > ; type NoteHeader = elf :: NoteHeader32 < Endian > ; type Dyn = elf :: Dyn64 < Endian > ; type Sym = elf :: Sym64 < Endian > ; type Rel = elf :: Rel64 < Endian > ; type Rela = elf :: Rela64 < Endian > ; type Relr = elf :: Relr64 < Endian > ; # [inline] fn is_type_64 (& self) -> bool { true } # [inline] fn is_type_64_sized () -> bool where Self : Sized , { true } # [inline] fn e_ident (& self) -> & elf :: Ident { & self . e_ident } # [inline] fn e_type (& self , endian : Self :: Endian) -> u16 { self . e_type . get (endian) } # [inline] fn e_machine (& self , endian : Self :: Endian) -> u16 { self . e_machine . get (endian) } # [inline] fn e_version (& self , endian : Self :: Endian) -> u32 { self . e_version . get (endian) } # [inline] fn e_entry (& self , endian : Self :: Endian) -> Self :: Word { self . e_entry . get (endian) } # [inline] fn e_phoff (& self , endian : Self :: Endian) -> Self :: Word { self . e_phoff . get (endian) } # [inline] fn e_shoff (& self , endian : Self :: Endian) -> Self :: Word { self . e_shoff . get (endian) } # [inline] fn e_flags (& self , endian : Self :: Endian) -> u32 { self . e_flags . get (endian) } # [inline] fn e_ehsize (& self , endian : Self :: Endian) -> u16 { self . e_ehsize . get (endian) } # [inline] fn e_phentsize (& self , endian : Self :: Endian) -> u16 { self . e_phentsize . get (endian) } # [inline] fn e_phnum (& self , endian : Self :: Endian) -> u16 { self . e_phnum . get (endian) } # [inline] fn e_shentsize (& self , endian : Self :: Endian) -> u16 { self . e_shentsize . get (endian) } # [inline] fn e_shnum (& self , endian : Self :: Endian) -> u16 { self . e_shnum . get (endian) } # [inline] fn e_shstrndx (& self , endian : Self :: Endian) -> u16 { self . e_shstrndx . get (endian) } }
    };
}

impl_287!()