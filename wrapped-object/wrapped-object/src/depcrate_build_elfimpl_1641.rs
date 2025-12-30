// Generated macro for impl_1641 (impl)
macro_rules! Depcrate_build_elfimpl_1641 {
() => {
// Module: crate::build::elf
// Provides: {"impl_1641"}
// Dependencies: {}
impl VersionId { # [doc = " Return `True` if this is a special version that does not exist in the version table."] pub fn is_special (& self) -> bool { self . 0 < VERSION_ID_BASE } # [doc = " Return the ID for a version index of [`elf::VER_NDX_LOCAL`]."] pub fn local () -> Self { VersionId (elf :: VER_NDX_LOCAL as usize) } # [doc = " Return the ID for a version index of [`elf::VER_NDX_GLOBAL`]."] pub fn global () -> Self { VersionId (elf :: VER_NDX_GLOBAL as usize) } }
};
}
