macro_rules! deps {
    () => {
        HashHeader!();
        FileHeader64!();
        Relr32!();
        GnuHashHeader!();
        Class!();
        Sym64!();
        Rel32!();
        Verdef!();
        Verdaux!();
        Rel64!();
        ProgramHeader32!();
        Endianness!();
        Rela32!();
        Verneed!();
        Vernaux!();
        SectionHeader32!();
        FileHeader32!();
        Sym32!();
        ProgramHeader64!();
        Rela64!();
        Dyn32!();
        SectionHeader64!();
        Relr64!();
        Dyn64!();
    };
}

macro_rules! impl_974 {
    () => {
        deps!();
        impl Class { # [doc = " Return the alignment size."] pub fn align (self) -> usize { if self . is_64 { 8 } else { 4 } } # [doc = " Return the size of the file header."] pub fn file_header_size (self) -> usize { if self . is_64 { mem :: size_of :: < elf :: FileHeader64 < Endianness > > () } else { mem :: size_of :: < elf :: FileHeader32 < Endianness > > () } } # [doc = " Return the size of a program header."] pub fn program_header_size (self) -> usize { if self . is_64 { mem :: size_of :: < elf :: ProgramHeader64 < Endianness > > () } else { mem :: size_of :: < elf :: ProgramHeader32 < Endianness > > () } } # [doc = " Return the size of a section header."] pub fn section_header_size (self) -> usize { if self . is_64 { mem :: size_of :: < elf :: SectionHeader64 < Endianness > > () } else { mem :: size_of :: < elf :: SectionHeader32 < Endianness > > () } } # [doc = " Return the size of a symbol."] pub fn sym_size (self) -> usize { if self . is_64 { mem :: size_of :: < elf :: Sym64 < Endianness > > () } else { mem :: size_of :: < elf :: Sym32 < Endianness > > () } } # [doc = " Return the size of a relocation entry."] pub fn rel_size (self , is_rela : bool) -> usize { if self . is_64 { if is_rela { mem :: size_of :: < elf :: Rela64 < Endianness > > () } else { mem :: size_of :: < elf :: Rel64 < Endianness > > () } } else { if is_rela { mem :: size_of :: < elf :: Rela32 < Endianness > > () } else { mem :: size_of :: < elf :: Rel32 < Endianness > > () } } } # [doc = " Return the size of a relative relocation entry."] pub fn relr_size (self) -> usize { if self . is_64 { mem :: size_of :: < elf :: Relr64 < Endianness > > () } else { mem :: size_of :: < elf :: Relr32 < Endianness > > () } } # [doc = " Return the size of a dynamic entry."] pub fn dyn_size (self) -> usize { if self . is_64 { mem :: size_of :: < elf :: Dyn64 < Endianness > > () } else { mem :: size_of :: < elf :: Dyn32 < Endianness > > () } } # [doc = " Return the size of a hash table."] pub fn hash_size (self , bucket_count : u32 , chain_count : u32) -> usize { mem :: size_of :: < elf :: HashHeader < Endianness > > () + bucket_count as usize * 4 + chain_count as usize * 4 } # [doc = " Return the size of a GNU hash table."] pub fn gnu_hash_size (self , bloom_count : u32 , bucket_count : u32 , symbol_count : u32) -> usize { let bloom_size = if self . is_64 { 8 } else { 4 } ; mem :: size_of :: < elf :: GnuHashHeader < Endianness > > () + bloom_count as usize * bloom_size + bucket_count as usize * 4 + symbol_count as usize * 4 } # [doc = " Return the size of a GNU symbol version section."] pub fn gnu_versym_size (self , symbol_count : usize) -> usize { symbol_count * 2 } # [doc = " Return the size of a GNU version definition section."] pub fn gnu_verdef_size (self , verdef_count : usize , verdaux_count : usize) -> usize { verdef_count * mem :: size_of :: < elf :: Verdef < Endianness > > () + verdaux_count * mem :: size_of :: < elf :: Verdaux < Endianness > > () } # [doc = " Return the size of a GNU version dependency section."] pub fn gnu_verneed_size (self , verneed_count : usize , vernaux_count : usize) -> usize { verneed_count * mem :: size_of :: < elf :: Verneed < Endianness > > () + vernaux_count * mem :: size_of :: < elf :: Vernaux < Endianness > > () } }
    };
}

impl_974!()