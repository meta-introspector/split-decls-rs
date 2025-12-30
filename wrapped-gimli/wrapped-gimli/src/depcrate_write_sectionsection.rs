// Generated macro for Section (trait)
macro_rules! Depcrate_write_sectionSection {
() => {
// Module: crate::write::section
// Provides: {"Section"}
// Dependencies: {}
# [doc = " Functionality common to all writable DWARF sections."] pub trait Section < W : Writer > : DerefMut < Target = W > { # [doc = " Returns the DWARF section kind for this type."] fn id (& self) -> SectionId ; # [doc = " Returns the ELF section name for this type."] fn name (& self) -> & 'static str { self . id () . name () } }
};
}
