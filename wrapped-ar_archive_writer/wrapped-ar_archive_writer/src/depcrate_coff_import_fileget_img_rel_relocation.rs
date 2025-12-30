// Generated macro for get_img_rel_relocation (function)
macro_rules! Depcrate_coff_import_fileget_img_rel_relocation {
() => {
// Module: crate::coff_import_file
// Provides: {"get_img_rel_relocation"}
// Dependencies: {}
fn get_img_rel_relocation (machine : MachineTypes) -> object :: U16 < object :: LittleEndian > { u16 ! (match machine { MachineTypes :: AMD64 => IMAGE_REL_AMD64_ADDR32NB , MachineTypes :: ARMNT => IMAGE_REL_ARM_ADDR32NB , MachineTypes :: ARM64 | MachineTypes :: ARM64EC | MachineTypes :: ARM64X => IMAGE_REL_ARM64_ADDR32NB , MachineTypes :: I386 => IMAGE_REL_I386_DIR32NB , }) }
};
}
