// Generated macro for impl_618 (impl)
macro_rules! Depcrate_read_elf_versionimpl_618 {
() => {
// Module: crate::read::elf::version
// Provides: {"impl_618"}
// Dependencies: {}
impl VersionIndex { # [doc = " Return the version index."] pub fn index (& self) -> u16 { self . 0 & elf :: VERSYM_VERSION } # [doc = " Return true if it is the local index."] pub fn is_local (& self) -> bool { self . index () == elf :: VER_NDX_LOCAL } # [doc = " Return true if it is the global index."] pub fn is_global (& self) -> bool { self . index () == elf :: VER_NDX_GLOBAL } # [doc = " Return the hidden flag."] pub fn is_hidden (& self) -> bool { self . 0 & elf :: VERSYM_HIDDEN != 0 } }
};
}
