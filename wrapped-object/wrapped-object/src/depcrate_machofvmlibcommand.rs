// Generated macro for FvmlibCommand (struct)
macro_rules! Depcrate_machoFvmlibCommand {
() => {
// Module: crate::macho
// Provides: {"FvmlibCommand"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct FvmlibCommand < E : Endian > { # [doc = " LC_IDFVMLIB or LC_LOADFVMLIB"] pub cmd : U32 < E > , # [doc = " includes pathname string"] pub cmdsize : U32 < E > , # [doc = " the library identification"] pub fvmlib : Fvmlib < E > , }
};
}
