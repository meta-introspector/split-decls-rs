macro_rules! deps {
    () => {
        SectionHeader!();
        FileHeader!();
        CompressionHeader!();
        CompressionHeader32!();
        NoteHeader32!();
        Rel!();
        Relr!();
        FileHeader32!();
        Sym!();
        ProgramHeader!();
        NoteHeader!();
        SectionHeader32!();
        Rel32!();
        Relr32!();
        Dyn!();
        ProgramHeader32!();
        Rela!();
        Sym32!();
        Ident!();
        Dyn32!();
        Endian!();
        Rela32!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > FileHeader for elf :: FileHeader32 < Endian > { type Word = u32 ; type Sword = i32 ; type Endian = Endian ; type ProgramHeader = elf :: ProgramHeader32 < Endian > ; type SectionHeader = elf :: SectionHeader32 < Endian > ; type CompressionHeader = elf :: CompressionHeader32 < Endian > ; type NoteHeader = elf :: NoteHeader32 < Endian > ; type Dyn = elf :: Dyn32 < Endian > ; type Sym = elf :: Sym32 < Endian > ; type Rel = elf :: Rel32 < Endian > ; type Rela = elf :: Rela32 < Endian > ; type Relr = elf :: Relr32 < Endian > ; # [inline] fn is_type_64 (& self) -> bool { false } # [inline] fn is_type_64_sized () -> bool where Self : Sized , { false } # [inline] fn e_ident (& self) -> & elf :: Ident { & self . e_ident } # [inline] fn e_type (& self , endian : Self :: Endian) -> u16 { self . e_type . get (endian) } # [inline] fn e_machine (& self , endian : Self :: Endian) -> u16 { self . e_machine . get (endian) } # [inline] fn e_version (& self , endian : Self :: Endian) -> u32 { self . e_version . get (endian) } # [inline] fn e_entry (& self , endian : Self :: Endian) -> Self :: Word { self . e_entry . get (endian) } # [inline] fn e_phoff (& self , endian : Self :: Endian) -> Self :: Word { self . e_phoff . get (endian) } # [inline] fn e_shoff (& self , endian : Self :: Endian) -> Self :: Word { self . e_shoff . get (endian) } # [inline] fn e_flags (& self , endian : Self :: Endian) -> u32 { self . e_flags . get (endian) } # [inline] fn e_ehsize (& self , endian : Self :: Endian) -> u16 { self . e_ehsize . get (endian) } # [inline] fn e_phentsize (& self , endian : Self :: Endian) -> u16 { self . e_phentsize . get (endian) } # [inline] fn e_phnum (& self , endian : Self :: Endian) -> u16 { self . e_phnum . get (endian) } # [inline] fn e_shentsize (& self , endian : Self :: Endian) -> u16 { self . e_shentsize . get (endian) } # [inline] fn e_shnum (& self , endian : Self :: Endian) -> u16 { self . e_shnum . get (endian) } # [inline] fn e_shstrndx (& self , endian : Self :: Endian) -> u16 { self . e_shstrndx . get (endian) } }
    };
}

impl_286!()