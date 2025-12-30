// Generated macro for Relocation (struct)
macro_rules! Depcrate_build_elfRelocation {
() => {
// Module: crate::build::elf
// Provides: {"Relocation"}
// Dependencies: {}
# [doc = " A relocation stored in a [`Section`]."] # [doc = ""] # [doc = " This corresponds to [`elf::Rel32`], [`elf::Rela32`], [`elf::Rel64`] or [`elf::Rela64`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Relocation < const DYNAMIC : bool = false > { # [doc = " The `r_offset` field in the ELF relocation."] pub r_offset : u64 , # [doc = " The symbol referenced by the ELF relocation."] pub symbol : Option < SymbolId < DYNAMIC > > , # [doc = " The `r_type` field in the ELF relocation."] pub r_type : u32 , # [doc = " The `r_addend` field in the ELF relocation."] # [doc = ""] # [doc = " Only used if the section type is `SHT_RELA`."] pub r_addend : i64 , }
};
}
