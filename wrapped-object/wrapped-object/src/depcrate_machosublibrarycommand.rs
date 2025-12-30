// Generated macro for SubLibraryCommand (struct)
macro_rules! Depcrate_machoSubLibraryCommand {
() => {
// Module: crate::macho
// Provides: {"SubLibraryCommand"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct SubLibraryCommand < E : Endian > { # [doc = " LC_SUB_LIBRARY"] pub cmd : U32 < E > , # [doc = " includes sub_library string"] pub cmdsize : U32 < E > , # [doc = " the sub_library name"] pub sub_library : LcStr < E > , }
};
}
