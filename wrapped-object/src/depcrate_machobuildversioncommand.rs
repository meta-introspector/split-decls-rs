// Generated macro for BuildVersionCommand (struct)
macro_rules! Depcrate_machoBuildVersionCommand {
() => {
// Module: crate::macho
// Provides: {"BuildVersionCommand"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct BuildVersionCommand < E : Endian > { # [doc = " LC_BUILD_VERSION"] pub cmd : U32 < E > , # [doc = " sizeof(struct BuildVersionCommand) plus ntools * sizeof(struct BuildToolVersion)"] pub cmdsize : U32 < E > , # [doc = " platform"] pub platform : U32 < E > , # [doc = " X.Y.Z is encoded in nibbles xxxx.yy.zz"] pub minos : U32 < E > , # [doc = " X.Y.Z is encoded in nibbles xxxx.yy.zz"] pub sdk : U32 < E > , # [doc = " number of tool entries following this"] pub ntools : U32 < E > , }
};
}
