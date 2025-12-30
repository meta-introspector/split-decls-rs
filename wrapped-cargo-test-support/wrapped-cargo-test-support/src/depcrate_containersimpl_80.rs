// Generated macro for impl_80 (impl)
macro_rules! Depcrate_containersimpl_80 {
() => {
// Module: crate::containers
// Provides: {"impl_80"}
// Dependencies: {}
impl MkFile { # [doc = " Defines a file to add to the container."] # [doc = ""] # [doc = " This should be passed to `Container::file`."] # [doc = ""] # [doc = " The path is the path inside the container to create the file."] pub fn path (path : & str) -> MkFile { MkFile { path : path . to_string () , contents : Vec :: new () , header : Header :: new_gnu () , } } pub fn contents (mut self , contents : impl Into < Vec < u8 > >) -> Self { self . contents = contents . into () ; self . header . set_size (self . contents . len () as u64) ; self } pub fn mode (mut self , mode : u32) -> Self { self . header . set_mode (mode) ; self } pub fn uid (mut self , uid : u64) -> Self { self . header . set_uid (uid) ; self } pub fn gid (mut self , gid : u64) -> Self { self . header . set_gid (gid) ; self } }
};
}
