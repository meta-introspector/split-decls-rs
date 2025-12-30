// Generated macro for VersionTable (struct)
macro_rules! Depcrate_read_elf_versionVersionTable {
() => {
// Module: crate::read::elf::version
// Provides: {"VersionTable"}
// Dependencies: {}
# [doc = " A table of version definitions and requirements."] # [doc = ""] # [doc = " It allows looking up the version information for a given symbol index."] # [doc = ""] # [doc = " This is derived from entries in the [`elf::SHT_GNU_VERSYM`], [`elf::SHT_GNU_VERDEF`]"] # [doc = " and [`elf::SHT_GNU_VERNEED`] sections."] # [doc = ""] # [doc = " Returned by [`SectionTable::versions`](super::SectionTable::versions)."] # [derive (Debug , Clone)] pub struct VersionTable < 'data , Elf : FileHeader > { symbols : & 'data [elf :: Versym < Elf :: Endian >] , versions : Vec < Version < 'data > > , }
};
}
