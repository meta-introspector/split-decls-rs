macro_rules! deps {
    () => {
        Result!();
        SymbolTable!();
        Pod!();
        ImageFileHeader!();
        ReadRef!();
        DataDirectories!();
        ImageThunkData!();
        Error!();
        ImageOptionalHeader!();
        SectionTable!();
    };
}

macro_rules! ImageNtHeaders {
    () => {
        deps!();
        # [doc = " A trait for generic access to [`pe::ImageNtHeaders32`] and [`pe::ImageNtHeaders64`]."] # [allow (missing_docs)] pub trait ImageNtHeaders : Debug + Pod { type ImageOptionalHeader : ImageOptionalHeader ; type ImageThunkData : ImageThunkData ; # [doc = " Return true if this type is a 64-bit header."] # [doc = ""] # [doc = " This is a property of the type, not a value in the header data."] fn is_type_64 (& self) -> bool ; # [doc = " Return true if the magic field in the optional header is valid."] fn is_valid_optional_magic (& self) -> bool ; # [doc = " Return the signature"] fn signature (& self) -> u32 ; # [doc = " Return the file header."] fn file_header (& self) -> & pe :: ImageFileHeader ; # [doc = " Return the optional header."] fn optional_header (& self) -> & Self :: ImageOptionalHeader ; # [doc = " Read the NT headers, including the data directories."] # [doc = ""] # [doc = " `data` must be for the entire file."] # [doc = ""] # [doc = " `offset` must be headers offset, which can be obtained from [`pe::ImageDosHeader::nt_headers_offset`]."] # [doc = " It is updated to point after the optional header, which is where the section headers are located."] # [doc = ""] # [doc = " Also checks that the `signature` and `magic` fields in the headers are valid."] fn parse < 'data , R : ReadRef < 'data > > (data : R , offset : & mut u64 ,) -> read :: Result < (& 'data Self , DataDirectories < 'data >) > { let nt_headers = data . read :: < Self > (offset) . read_error ("Invalid PE headers offset or size") ? ; if nt_headers . signature () != pe :: IMAGE_NT_SIGNATURE { return Err (Error ("Invalid PE magic")) ; } if ! nt_headers . is_valid_optional_magic () { return Err (Error ("Invalid PE optional header magic")) ; } let optional_data_size = u64 :: from (nt_headers . file_header () . size_of_optional_header . get (LE)) . checked_sub (mem :: size_of :: < Self :: ImageOptionalHeader > () as u64) . read_error ("PE optional header size is too small") ? ; let optional_data = data . read_bytes (offset , optional_data_size) . read_error ("Invalid PE optional header size") ? ; let data_directories = DataDirectories :: parse (optional_data , nt_headers . optional_header () . number_of_rva_and_sizes () ,) ? ; Ok ((nt_headers , data_directories)) } # [doc = " Read the section table."] # [doc = ""] # [doc = " `data` must be for the entire file."] # [doc = " `offset` must be after the optional file header."] # [inline] fn sections < 'data , R : ReadRef < 'data > > (& self , data : R , offset : u64 ,) -> read :: Result < SectionTable < 'data > > { SectionTable :: parse (self . file_header () , data , offset) } # [doc = " Read the COFF symbol table and string table."] # [doc = ""] # [doc = " `data` must be the entire file data."] # [inline] fn symbols < 'data , R : ReadRef < 'data > > (& self , data : R) -> read :: Result < SymbolTable < 'data , R > > { SymbolTable :: parse (self . file_header () , data) } }
    };
}

ImageNtHeaders!();