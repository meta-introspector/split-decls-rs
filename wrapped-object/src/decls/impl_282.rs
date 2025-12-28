macro_rules! deps {
    () => {
        ReadRef!();
        RelocationSections!();
        ElfFile!();
        FileHeader!();
        ProgramHeader!();
        ElfSection!();
        SectionTable!();
        Endian!();
        SymbolTable!();
        Result!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl < 'data , Elf , R > ElfFile < 'data , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { # [doc = " Parse the raw ELF file data."] pub fn parse (data : R) -> read :: Result < Self > { let header = Elf :: parse (data) ? ; let endian = header . endian () ? ; let segments = header . program_headers (endian , data) ? ; let sections = header . sections (endian , data) ? ; let symbols = sections . symbols (endian , data , elf :: SHT_SYMTAB) ? ; let dynamic_symbols = sections . symbols (endian , data , elf :: SHT_DYNSYM) ? ; let relocations = sections . relocation_sections (endian , symbols . section ()) ? ; Ok (ElfFile { endian , data , header , segments , sections , relocations , symbols , dynamic_symbols , }) } # [doc = " Returns the endianness."] pub fn endian (& self) -> Elf :: Endian { self . endian } # [doc = " Returns the raw data."] pub fn data (& self) -> R { self . data } # [doc = " Returns the raw ELF file header."] # [deprecated (note = "Use `elf_header` instead")] pub fn raw_header (& self) -> & 'data Elf { self . header } # [doc = " Returns the raw ELF segments."] # [deprecated (note = "Use `elf_program_headers` instead")] pub fn raw_segments (& self) -> & 'data [Elf :: ProgramHeader] { self . segments } # [doc = " Get the raw ELF file header."] pub fn elf_header (& self) -> & 'data Elf { self . header } # [doc = " Get the raw ELF program headers."] # [doc = ""] # [doc = " Returns an empty slice if the file has no program headers."] pub fn elf_program_headers (& self) -> & 'data [Elf :: ProgramHeader] { self . segments } # [doc = " Get the ELF section table."] # [doc = ""] # [doc = " Returns an empty section table if the file has no section headers."] pub fn elf_section_table (& self) -> & SectionTable < 'data , Elf , R > { & self . sections } # [doc = " Get the ELF symbol table."] # [doc = ""] # [doc = " Returns an empty symbol table if the file has no symbol table."] pub fn elf_symbol_table (& self) -> & SymbolTable < 'data , Elf , R > { & self . symbols } # [doc = " Get the ELF dynamic symbol table."] # [doc = ""] # [doc = " Returns an empty symbol table if the file has no dynamic symbol table."] pub fn elf_dynamic_symbol_table (& self) -> & SymbolTable < 'data , Elf , R > { & self . dynamic_symbols } # [doc = " Get a mapping for linked relocation sections."] pub fn elf_relocation_sections (& self) -> & RelocationSections { & self . relocations } fn raw_section_by_name < 'file > (& 'file self , section_name : & [u8] ,) -> Option < ElfSection < 'data , 'file , Elf , R > > { self . sections . section_by_name (self . endian , section_name) . map (| (index , section) | ElfSection { file : self , index , section , }) } # [cfg (feature = "compression")] fn zdebug_section_by_name < 'file > (& 'file self , section_name : & [u8] ,) -> Option < ElfSection < 'data , 'file , Elf , R > > { if ! section_name . starts_with (b".debug_") { return None ; } let mut name = Vec :: with_capacity (section_name . len () + 1) ; name . extend_from_slice (b".zdebug_") ; name . extend_from_slice (& section_name [7 ..]) ; self . raw_section_by_name (& name) } # [cfg (not (feature = "compression"))] fn zdebug_section_by_name < 'file > (& 'file self , _section_name : & [u8] ,) -> Option < ElfSection < 'data , 'file , Elf , R > > { None } }
    };
}

impl_282!()