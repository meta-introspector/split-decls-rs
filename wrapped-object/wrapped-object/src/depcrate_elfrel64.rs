// Generated macro for Rel64 (struct)
macro_rules! Depcrate_elfRel64 {
() => {
// Module: crate::elf
// Provides: {"Rel64"}
// Dependencies: {}
# [doc = " Relocation table entry without explicit addend."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Rel64 < E : Endian > { # [doc = " Relocation address."] pub r_offset : U64 < E > , # [doc = " Relocation type and symbol index."] pub r_info : U64 < E > , }
};
}
