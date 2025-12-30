// Generated macro for MachineTypes (enum)
macro_rules! Depcrate_coffMachineTypes {
() => {
// Module: crate::coff
// Provides: {"MachineTypes"}
// Dependencies: {}
# [derive (PartialEq , Eq , Copy , Clone , Debug)] # [repr (u16)] # [allow (clippy :: upper_case_acronyms)] pub enum MachineTypes { AMD64 = object :: pe :: IMAGE_FILE_MACHINE_AMD64 , ARMNT = object :: pe :: IMAGE_FILE_MACHINE_ARMNT , ARM64 = object :: pe :: IMAGE_FILE_MACHINE_ARM64 , ARM64EC = object :: pe :: IMAGE_FILE_MACHINE_ARM64EC , ARM64X = 0xA64E , I386 = object :: pe :: IMAGE_FILE_MACHINE_I386 , }
};
}
