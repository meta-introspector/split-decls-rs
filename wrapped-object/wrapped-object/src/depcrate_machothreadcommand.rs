// Generated macro for ThreadCommand (struct)
macro_rules! Depcrate_machoThreadCommand {
() => {
// Module: crate::macho
// Provides: {"ThreadCommand"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ThreadCommand < E : Endian > { # [doc = " LC_THREAD or  LC_UNIXTHREAD"] pub cmd : U32 < E > , # [doc = " total size of this command"] pub cmdsize : U32 < E > , }
};
}
