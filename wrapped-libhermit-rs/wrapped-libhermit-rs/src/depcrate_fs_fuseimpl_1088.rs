// Generated macro for impl_1088 (impl)
macro_rules! Depcrate_fs_fuseimpl_1088 {
() => {
// Module: crate::fs::fuse
// Provides: {"impl_1088"}
// Dependencies: {}
impl < O : ops :: Op > Cmd < O > where O : ops :: Op < InPayload = CString > , { fn with_cstring (nodeid : u64 , op_header : O :: InStruct , cstring : CString) -> Self { let cstring_bytes = cstring . into_bytes_with_nul () . to_vec_in (DeviceAlloc) ; Self { headers : Box :: new_in (CmdHeader :: with_payload_size (nodeid , op_header , cstring_bytes . len ()) , DeviceAlloc ,) , payload : Some (cstring_bytes) , } } }
};
}
