// Generated macro for Rela32 (struct)
macro_rules! Depcrate_elfRela32 {
() => {
// Module: crate::elf
// Provides: {"Rela32"}
// Dependencies: {}
# [doc = " Relocation table entry with explicit addend."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Rela32 < E : Endian > { # [doc = " Relocation address."] pub r_offset : U32 < E > , # [doc = " Relocation type and symbol index."] pub r_info : U32 < E > , # [doc = " Explicit addend."] pub r_addend : I32 < E > , }
};
}
