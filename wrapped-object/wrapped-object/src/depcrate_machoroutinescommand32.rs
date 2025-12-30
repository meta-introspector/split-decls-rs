// Generated macro for RoutinesCommand32 (struct)
macro_rules! Depcrate_machoRoutinesCommand32 {
() => {
// Module: crate::macho
// Provides: {"RoutinesCommand32"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct RoutinesCommand32 < E : Endian > { # [doc = " LC_ROUTINES"] pub cmd : U32 < E > , # [doc = " total size of this command"] pub cmdsize : U32 < E > , # [doc = " address of initialization routine"] pub init_address : U32 < E > , # [doc = " index into the module table that the init routine is defined in"] pub init_module : U32 < E > , pub reserved1 : U32 < E > , pub reserved2 : U32 < E > , pub reserved3 : U32 < E > , pub reserved4 : U32 < E > , pub reserved5 : U32 < E > , pub reserved6 : U32 < E > , }
};
}
