// Generated macro for FuseInterface (trait)
macro_rules! Depcrate_fs_fuseFuseInterface {
() => {
// Module: crate::fs::fuse
// Provides: {"FuseInterface"}
// Dependencies: {}
pub (crate) trait FuseInterface { fn send_command < O : ops :: Op + 'static > (& mut self , cmd : Cmd < O > , rsp_payload_len : u32 ,) -> Result < Rsp < O > , FuseError > where < O as ops :: Op > :: InStruct : Send , < O as ops :: Op > :: OutStruct : Send ; fn get_mount_point (& self) -> String ; }
};
}
