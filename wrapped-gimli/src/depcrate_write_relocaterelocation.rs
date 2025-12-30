// Generated macro for Relocation (struct)
macro_rules! Depcrate_write_relocateRelocation {
() => {
// Module: crate::write::relocate
// Provides: {"Relocation"}
// Dependencies: {}
# [doc = " A relocation to be applied to a section."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Relocation { # [doc = " The offset within the section where the relocation should be applied."] pub offset : usize , # [doc = " The size of the value to be relocated."] pub size : u8 , # [doc = " The target of the relocation."] pub target : RelocationTarget , # [doc = " The addend to be applied to the relocated value."] pub addend : i64 , # [doc = " The pointer encoding for relocations in unwind information."] pub eh_pe : Option < constants :: DwEhPe > , }
};
}
