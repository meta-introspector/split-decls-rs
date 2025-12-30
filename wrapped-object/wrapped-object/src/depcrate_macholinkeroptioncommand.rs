// Generated macro for LinkerOptionCommand (struct)
macro_rules! Depcrate_machoLinkerOptionCommand {
() => {
// Module: crate::macho
// Provides: {"LinkerOptionCommand"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct LinkerOptionCommand < E : Endian > { # [doc = " LC_LINKER_OPTION only used in MH_OBJECT filetypes"] pub cmd : U32 < E > , pub cmdsize : U32 < E > , # [doc = " number of strings"] pub count : U32 < E > , }
};
}
