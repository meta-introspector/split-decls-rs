// Generated macro for impl_1085 (impl)
macro_rules! Depcrate_fs_fuseimpl_1085 {
() => {
// Module: crate::fs::fuse
// Provides: {"impl_1085"}
// Dependencies: {}
impl < O : ops :: Op > CmdHeader < O > { fn with_payload_size (nodeid : u64 , op_header : O :: InStruct , len : usize) -> CmdHeader < O > { CmdHeader { in_header : fuse_in_header { len : (core :: mem :: size_of :: < fuse_in_header > () + core :: mem :: size_of :: < O :: InStruct > () + len) . try_into () . expect ("The command is too large") , opcode : O :: OP_CODE . into () , nodeid , unique : 1 , .. Default :: default () } , op_header , } } }
};
}
