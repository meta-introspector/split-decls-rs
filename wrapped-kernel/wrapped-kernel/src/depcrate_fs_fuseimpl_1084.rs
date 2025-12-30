// Generated macro for impl_1084 (impl)
macro_rules! Depcrate_fs_fuseimpl_1084 {
() => {
// Module: crate::fs::fuse
// Provides: {"impl_1084"}
// Dependencies: {}
impl < O : ops :: Op > CmdHeader < O > where O : ops :: Op < InPayload = () > , { fn new (nodeid : u64 , op_header : O :: InStruct) -> Self { Self :: with_payload_size (nodeid , op_header , 0) } }
};
}
