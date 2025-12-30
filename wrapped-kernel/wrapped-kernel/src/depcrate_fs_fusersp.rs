// Generated macro for Rsp (struct)
macro_rules! Depcrate_fs_fuseRsp {
() => {
// Module: crate::fs::fuse
// Provides: {"Rsp"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct Rsp < O : ops :: Op > { pub headers : Box < RspHeader < O > , DeviceAlloc > , pub payload : Option < Vec < u8 , DeviceAlloc > > , }
};
}
