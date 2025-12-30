// Generated macro for impl_1149 (impl)
macro_rules! Depcrate_read_xcoff_sectionimpl_1149 {
() => {
// Module: crate::read::xcoff::section
// Provides: {"impl_1149"}
// Dependencies: {}
impl < 'data , 'file , Xcoff : FileHeader , R : ReadRef < 'data > > XcoffSection < 'data , 'file , Xcoff , R > { # [doc = " Get the XCOFF file containing this section."] pub fn xcoff_file (& self) -> & 'file XcoffFile < 'data , Xcoff , R > { self . file } # [doc = " Get the raw XCOFF section header."] pub fn xcoff_section (& self) -> & 'data Xcoff :: SectionHeader { self . section } # [doc = " Get the raw XCOFF relocation entries for this section."] pub fn xcoff_relocations (& self) -> Result < & 'data [Xcoff :: Rel] > { self . section . relocations (self . file . data) } fn bytes (& self) -> Result < & 'data [u8] > { self . section . data (self . file . data) . read_error ("Invalid XCOFF section offset or size") } }
};
}
