// Generated macro for macro_302 (macro)
macro_rules! Depcrate_unistdmacro_302 {
() => {
// Module: crate::unistd
// Provides: {"macro_302"}
// Dependencies: {}
feature ! { #! [all (feature = "socket" , feature = "user")] # [doc = " Get the effective user ID and group ID associated with a Unix domain socket."] # [doc = ""] # [doc = " See also [getpeereid(3)](https://www.freebsd.org/cgi/man.cgi?query=getpeereid)"] # [cfg (bsd)] pub fn getpeereid < F : std :: os :: fd :: AsFd > (fd : F) -> Result < (Uid , Gid) > { use std :: os :: fd :: AsRawFd ; let mut uid = 1 ; let mut gid = 1 ; let ret = unsafe { libc :: getpeereid (fd . as_fd () . as_raw_fd () , & mut uid , & mut gid) } ; Errno :: result (ret) . map (| _ | (Uid (uid) , Gid (gid))) } }
};
}
