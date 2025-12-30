// Generated macro for PrebindCksumCommand (struct)
macro_rules! Depcrate_machoPrebindCksumCommand {
() => {
// Module: crate::macho
// Provides: {"PrebindCksumCommand"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct PrebindCksumCommand < E : Endian > { # [doc = " LC_PREBIND_CKSUM"] pub cmd : U32 < E > , # [doc = " sizeof(struct PrebindCksumCommand)"] pub cmdsize : U32 < E > , # [doc = " the check sum or zero"] pub cksum : U32 < E > , }
};
}
