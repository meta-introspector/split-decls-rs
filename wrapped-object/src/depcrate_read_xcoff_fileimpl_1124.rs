// Generated macro for impl_1124 (impl)
macro_rules! Depcrate_read_xcoff_fileimpl_1124 {
() => {
// Module: crate::read::xcoff::file
// Provides: {"impl_1124"}
// Dependencies: {}
impl < 'data , Xcoff , R > XcoffFile < 'data , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { # [doc = " Parse the raw XCOFF file data."] pub fn parse (data : R) -> Result < Self > { let mut offset = 0 ; let header = Xcoff :: parse (data , & mut offset) ? ; let aux_header = header . aux_header (data , & mut offset) ? ; let sections = header . sections (data , & mut offset) ? ; let symbols = header . symbols (data) ? ; Ok (XcoffFile { data , header , aux_header , sections , symbols , }) } # [doc = " Returns the raw data."] pub fn data (& self) -> R { self . data } # [doc = " Returns the raw XCOFF file header."] # [deprecated (note = "Use `xcoff_header` instead")] pub fn raw_header (& self) -> & 'data Xcoff { self . header } # [doc = " Get the raw XCOFF file header."] pub fn xcoff_header (& self) -> & 'data Xcoff { self . header } # [doc = " Get the raw XCOFF auxiliary header."] pub fn xcoff_aux_header (& self) -> Option < & 'data Xcoff :: AuxHeader > { self . aux_header } # [doc = " Get the XCOFF section table."] pub fn xcoff_section_table (& self) -> & SectionTable < 'data , Xcoff > { & self . sections } # [doc = " Get the XCOFF symbol table."] pub fn xcoff_symbol_table (& self) -> & SymbolTable < 'data , Xcoff , R > { & self . symbols } }
};
}
