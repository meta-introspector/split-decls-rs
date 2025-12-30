// Generated macro for impl_51 (impl)
macro_rules! Depcrate_coffimpl_51 {
() => {
// Module: crate::coff
// Provides: {"impl_51"}
// Dependencies: {}
impl TryInto < MachineTypes > for u16 { type Error = () ; fn try_into (self) -> Result < MachineTypes , Self :: Error > { match self { object :: pe :: IMAGE_FILE_MACHINE_AMD64 => Ok (MachineTypes :: AMD64) , object :: pe :: IMAGE_FILE_MACHINE_ARMNT => Ok (MachineTypes :: ARMNT) , object :: pe :: IMAGE_FILE_MACHINE_ARM64 => Ok (MachineTypes :: ARM64) , object :: pe :: IMAGE_FILE_MACHINE_ARM64EC => Ok (MachineTypes :: ARM64EC) , 0xA64E => Ok (MachineTypes :: ARM64X) , object :: pe :: IMAGE_FILE_MACHINE_I386 => Ok (MachineTypes :: I386) , _ => Err (()) , } } }
};
}
