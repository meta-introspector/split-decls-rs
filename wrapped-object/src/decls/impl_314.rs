macro_rules! deps {
    () => {
        ElfSection!();
        ReadRef!();
        SectionIndex!();
        Result!();
        CompressedFileRange!();
        CompressionFormat!();
        SectionHeader!();
        Rel!();
        ElfFile!();
        Error!();
        Rela!();
        FileHeader!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl < 'data , 'file , Elf : FileHeader , R : ReadRef < 'data > > ElfSection < 'data , 'file , Elf , R > { # [doc = " Get the ELF file containing this section."] pub fn elf_file (& self) -> & 'file ElfFile < 'data , Elf , R > { self . file } # [doc = " Get the raw ELF section header."] pub fn elf_section_header (& self) -> & 'data Elf :: SectionHeader { self . section } # [doc = " Get the index of the relocation section that references this section."] # [doc = ""] # [doc = " Returns `None` if there are no relocations."] # [doc = " Returns an error if there are multiple relocation sections that reference this section."] pub fn elf_relocation_section_index (& self) -> read :: Result < Option < SectionIndex > > { let Some (relocation_index) = self . file . relocations . get (self . index) else { return Ok (None) ; } ; if self . file . relocations . get (relocation_index) . is_some () { return Err (Error ("Unsupported ELF section with multiple relocation sections" ,)) ; } Ok (Some (relocation_index)) } # [doc = " Get the relocation section that references this section."] # [doc = ""] # [doc = " Returns `None` if there are no relocations."] # [doc = " Returns an error if there are multiple relocation sections that reference this section."] pub fn elf_relocation_section (& self) -> read :: Result < Option < & 'data Elf :: SectionHeader > > { let Some (relocation_index) = self . elf_relocation_section_index () ? else { return Ok (None) ; } ; self . file . sections . section (relocation_index) . map (Some) } # [doc = " Get the `Elf::Rel` entries that apply to this section."] # [doc = ""] # [doc = " Returns an empty slice if there are no relocations."] # [doc = " Returns an error if there are multiple relocation sections that reference this section."] pub fn elf_linked_rel (& self) -> read :: Result < & 'data [Elf :: Rel] > { let Some (relocation_section) = self . elf_relocation_section () ? else { return Ok (& []) ; } ; let Some ((rel , _)) = relocation_section . rel (self . file . endian , self . file . data) ? else { return Ok (& []) ; } ; Ok (rel) } # [doc = " Get the `Elf::Rela` entries that apply to this section."] # [doc = ""] # [doc = " Returns an empty slice if there are no relocations."] # [doc = " Returns an error if there are multiple relocation sections that reference this section."] pub fn elf_linked_rela (& self) -> read :: Result < & 'data [Elf :: Rela] > { let Some (relocation_section) = self . elf_relocation_section () ? else { return Ok (& []) ; } ; let Some ((rela , _)) = relocation_section . rela (self . file . endian , self . file . data) ? else { return Ok (& []) ; } ; Ok (rela) } fn bytes (& self) -> read :: Result < & 'data [u8] > { self . section . data (self . file . endian , self . file . data) . read_error ("Invalid ELF section size or offset") } fn maybe_compressed (& self) -> read :: Result < Option < CompressedFileRange > > { let endian = self . file . endian ; if let Some ((header , offset , compressed_size)) = self . section . compression (endian , self . file . data) ? { let format = match header . ch_type (endian) { elf :: ELFCOMPRESS_ZLIB => CompressionFormat :: Zlib , elf :: ELFCOMPRESS_ZSTD => CompressionFormat :: Zstandard , _ => return Err (Error ("Unsupported ELF compression type")) , } ; let uncompressed_size = header . ch_size (endian) . into () ; Ok (Some (CompressedFileRange { format , offset , compressed_size , uncompressed_size , })) } else { Ok (None) } } fn maybe_compressed_gnu (& self) -> read :: Result < Option < CompressedFileRange > > { if ! self . name () . map_or (false , | name | name . starts_with (".zdebug_")) { return Ok (None) ; } let (section_offset , section_size) = self . file_range () . read_error ("Invalid ELF GNU compressed section type") ? ; gnu_compression :: compressed_file_range (self . file . data , section_offset , section_size) . map (Some) } }
    };
}

impl_314!()