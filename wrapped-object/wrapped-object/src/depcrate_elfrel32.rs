// Generated macro for Rel32 (struct)
macro_rules! Depcrate_elfRel32 {
() => {
// Module: crate::elf
// Provides: {"Rel32"}
// Dependencies: {}
# [doc = " Relocation table entry without explicit addend."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Rel32 < E : Endian > { # [doc = " Relocation address."] pub r_offset : U32 < E > , # [doc = " Relocation type and symbol index."] pub r_info : U32 < E > , }
};
}
