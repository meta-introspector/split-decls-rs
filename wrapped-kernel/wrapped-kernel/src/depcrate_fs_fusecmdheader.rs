// Generated macro for CmdHeader (struct)
macro_rules! Depcrate_fs_fuseCmdHeader {
() => {
// Module: crate::fs::fuse
// Provides: {"CmdHeader"}
// Dependencies: {}
# [repr (C)] # [derive (Debug)] pub (crate) struct CmdHeader < O : ops :: Op > { pub in_header : fuse_in_header , op_header : O :: InStruct , }
};
}
