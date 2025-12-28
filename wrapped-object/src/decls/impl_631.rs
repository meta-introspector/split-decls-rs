macro_rules! deps {
    () => {
        PeFile!();
        CoffCommon!();
        ImageNtHeaders!();
        ReadRef!();
        RichHeaderInfo!();
        ImportTable!();
        ImageDataDirectory!();
        DataDirectories!();
        ExportTable!();
        ImageDosHeader!();
        SectionTable!();
        Result!();
    };
}

macro_rules! impl_631 {
    () => {
        deps!();
        impl < 'data , Pe , R > PeFile < 'data , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { # [doc = " Parse the raw PE file data."] pub fn parse (data : R) -> Result < Self > { let dos_header = pe :: ImageDosHeader :: parse (data) ? ; let mut offset = dos_header . nt_headers_offset () . into () ; let (nt_headers , data_directories) = Pe :: parse (data , & mut offset) ? ; let sections = nt_headers . sections (data , offset) ? ; let coff_symbols = nt_headers . symbols (data) ; let image_base = nt_headers . optional_header () . image_base () ; Ok (PeFile { dos_header , nt_headers , data_directories , common : CoffCommon { sections , symbols : coff_symbols . unwrap_or_default () , image_base , } , data , }) } # [doc = " Returns this binary data."] pub fn data (& self) -> R { self . data } # [doc = " Return the DOS header of this file."] pub fn dos_header (& self) -> & 'data pe :: ImageDosHeader { self . dos_header } # [doc = " Return the NT Headers of this file."] pub fn nt_headers (& self) -> & 'data Pe { self . nt_headers } # [doc = " Returns information about the rich header of this file (if any)."] pub fn rich_header_info (& self) -> Option < RichHeaderInfo < '_ > > { RichHeaderInfo :: parse (self . data , self . dos_header . nt_headers_offset () . into ()) } # [doc = " Returns the section table of this binary."] pub fn section_table (& self) -> SectionTable < 'data > { self . common . sections } # [doc = " Returns the data directories of this file."] pub fn data_directories (& self) -> DataDirectories < 'data > { self . data_directories } # [doc = " Returns the data directory at the given index."] pub fn data_directory (& self , id : usize) -> Option < & 'data pe :: ImageDataDirectory > { self . data_directories . get (id) } # [doc = " Returns the export table of this file."] # [doc = ""] # [doc = " The export table is located using the data directory."] pub fn export_table (& self) -> Result < Option < ExportTable < 'data > > > { self . data_directories . export_table (self . data , & self . common . sections) } # [doc = " Returns the import table of this file."] # [doc = ""] # [doc = " The import table is located using the data directory."] pub fn import_table (& self) -> Result < Option < ImportTable < 'data > > > { self . data_directories . import_table (self . data , & self . common . sections) } pub (super) fn section_alignment (& self) -> u64 { u64 :: from (self . nt_headers . optional_header () . section_alignment ()) } }
    };
}

impl_631!();