// Generated macro for impl_1657 (impl)
macro_rules! Depcrate_build_elfimpl_1657 {
() => {
// Module: crate::build::elf
// Provides: {"impl_1657"}
// Dependencies: {}
impl AttributeTag { # [doc = " Return the corresponding `elf::Tag_*` value for this tag."] pub fn tag (& self) -> u8 { match self { AttributeTag :: File => elf :: Tag_File , AttributeTag :: Section (_) => elf :: Tag_Section , AttributeTag :: Symbol (_) => elf :: Tag_Symbol , } } }
};
}
