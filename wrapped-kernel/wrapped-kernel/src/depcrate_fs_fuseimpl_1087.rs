// Generated macro for impl_1087 (impl)
macro_rules! Depcrate_fs_fuseimpl_1087 {
() => {
// Module: crate::fs::fuse
// Provides: {"impl_1087"}
// Dependencies: {}
impl < O : ops :: Op > Cmd < O > where O : ops :: Op < InPayload = () > , { fn new (nodeid : u64 , op_header : O :: InStruct) -> Self { Self { headers : Box :: new_in (CmdHeader :: new (nodeid , op_header) , DeviceAlloc) , payload : None , } } }
};
}
