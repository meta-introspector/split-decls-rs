// Generated macro for Version (struct)
macro_rules! Depcrate_read_elf_versionVersion {
() => {
// Module: crate::read::elf::version
// Provides: {"Version"}
// Dependencies: {}
# [doc = " A version definition or requirement."] # [doc = ""] # [doc = " This is derived from entries in the [`elf::SHT_GNU_VERDEF`] and [`elf::SHT_GNU_VERNEED`] sections."] # [derive (Debug , Default , Clone , Copy)] pub struct Version < 'data > { name : & 'data [u8] , hash : u32 , valid : bool , file : Option < & 'data [u8] > , }
};
}
