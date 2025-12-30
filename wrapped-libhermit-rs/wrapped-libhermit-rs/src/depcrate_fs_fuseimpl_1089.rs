// Generated macro for impl_1089 (impl)
macro_rules! Depcrate_fs_fuseimpl_1089 {
() => {
// Module: crate::fs::fuse
// Provides: {"impl_1089"}
// Dependencies: {}
impl < O : ops :: Op > Cmd < O > where O : ops :: Op < InPayload = [u8] > , { fn with_boxed_slice (nodeid : u64 , op_header : O :: InStruct , slice : Box < [u8] >) -> Self { let mut device_slice = Vec :: with_capacity_in (slice . len () , DeviceAlloc) ; device_slice . extend_from_slice (& slice) ; Self { headers : Box :: new_in (CmdHeader :: with_payload_size (nodeid , op_header , slice . len ()) , DeviceAlloc ,) , payload : Some (device_slice) , } } }
};
}
