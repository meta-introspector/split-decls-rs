// Generated macro for impl_620 (impl)
macro_rules! Depcrate_read_elf_versionimpl_620 {
() => {
// Module: crate::read::elf::version
// Provides: {"impl_620"}
// Dependencies: {}
impl < 'data > Version < 'data > { # [doc = " Return the version name."] pub fn name (& self) -> & 'data [u8] { self . name } # [doc = " Return hash of the version name."] pub fn hash (& self) -> u32 { self . hash } # [doc = " Return the filename of the library containing this version."] # [doc = ""] # [doc = " This is the `vn_file` field of the associated entry in [`elf::SHT_GNU_VERNEED`]."] # [doc = " or `None` if the version info was parsed from a [`elf::SHT_GNU_VERDEF`] section."] pub fn file (& self) -> Option < & 'data [u8] > { self . file } }
};
}
