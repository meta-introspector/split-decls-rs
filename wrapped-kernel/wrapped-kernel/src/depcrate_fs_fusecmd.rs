// Generated macro for Cmd (struct)
macro_rules! Depcrate_fs_fuseCmd {
() => {
// Module: crate::fs::fuse
// Provides: {"Cmd"}
// Dependencies: {}
pub (crate) struct Cmd < O : ops :: Op > { pub headers : Box < CmdHeader < O > , DeviceAlloc > , pub payload : Option < Vec < u8 , DeviceAlloc > > , }
};
}
