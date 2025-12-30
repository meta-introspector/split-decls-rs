// Generated macro for RelocateReader (struct)
macro_rules! Depcrate_read_relocateRelocateReader {
() => {
// Module: crate::read::relocate
// Provides: {"RelocateReader"}
// Dependencies: {}
# [doc = " A `Reader` which applies relocations to addresses and offsets."] # [doc = ""] # [doc = " This is useful for reading sections which contain relocations,"] # [doc = " such as those in a relocatable object file."] # [doc = " It is generally not used for reading sections in an executable file."] # [derive (Debug , Clone)] pub struct RelocateReader < R : Reader < Offset = usize > , T : Relocate < R :: Offset > > { section : R , reader : R , relocate : T , }
};
}
