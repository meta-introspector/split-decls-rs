// Generated macro for IdentCommand (struct)
macro_rules! Depcrate_machoIdentCommand {
() => {
// Module: crate::macho
// Provides: {"IdentCommand"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct IdentCommand < E : Endian > { # [doc = " LC_IDENT"] pub cmd : U32 < E > , # [doc = " strings that follow this command"] pub cmdsize : U32 < E > , }
};
}
