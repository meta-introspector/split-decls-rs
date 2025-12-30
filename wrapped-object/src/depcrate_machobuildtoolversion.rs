// Generated macro for BuildToolVersion (struct)
macro_rules! Depcrate_machoBuildToolVersion {
() => {
// Module: crate::macho
// Provides: {"BuildToolVersion"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct BuildToolVersion < E : Endian > { # [doc = " enum for the tool"] pub tool : U32 < E > , # [doc = " version number of the tool"] pub version : U32 < E > , }
};
}
