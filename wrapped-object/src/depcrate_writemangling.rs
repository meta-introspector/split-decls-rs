// Generated macro for Mangling (enum)
macro_rules! Depcrate_writeMangling {
() => {
// Module: crate::write
// Provides: {"Mangling"}
// Dependencies: {}
# [doc = " The symbol name mangling scheme."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum Mangling { # [doc = " No symbol mangling."] None , # [doc = " Windows COFF symbol mangling."] Coff , # [doc = " Windows COFF i386 symbol mangling."] CoffI386 , # [doc = " ELF symbol mangling."] Elf , # [doc = " Mach-O symbol mangling."] MachO , # [doc = " Xcoff symbol mangling."] Xcoff , }
};
}
