// Generated macro for Rela64 (struct)
macro_rules! Depcrate_elfRela64 {
() => {
// Module: crate::elf
// Provides: {"Rela64"}
// Dependencies: {}
# [doc = " Relocation table entry with explicit addend."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Rela64 < E : Endian > { # [doc = " Relocation address."] pub r_offset : U64 < E > , # [doc = " Relocation type and symbol index."] pub r_info : U64 < E > , # [doc = " Explicit addend."] pub r_addend : I64 < E > , }
};
}
