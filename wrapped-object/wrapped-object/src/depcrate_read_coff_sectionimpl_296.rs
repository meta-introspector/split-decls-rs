// Generated macro for impl_296 (impl)
macro_rules! Depcrate_read_coff_sectionimpl_296 {
() => {
// Module: crate::read::coff::section
// Provides: {"impl_296"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > CoffSection < 'data , 'file , R , Coff > { # [doc = " Get the COFF file containing this section."] pub fn coff_file (& self) -> & 'file CoffFile < 'data , R , Coff > { self . file } # [doc = " Get the raw COFF section header."] pub fn coff_section (& self) -> & 'data pe :: ImageSectionHeader { self . section } # [doc = " Get the raw COFF relocations for this section."] pub fn coff_relocations (& self) -> Result < & 'data [pe :: ImageRelocation] > { self . section . coff_relocations (self . file . data) } fn bytes (& self) -> Result < & 'data [u8] > { self . section . coff_data (self . file . data) . read_error ("Invalid COFF section offset or size") } }
};
}
