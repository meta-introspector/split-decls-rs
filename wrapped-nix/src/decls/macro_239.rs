macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! macro_239 {
    () => {
        deps!();
        feature ! { #! [feature = "term"] # [doc = " Get the name of the terminal device that is open on file descriptor fd"] # [doc = " (see [`ttyname(3)`](https://man7.org/linux/man-pages/man3/ttyname.3.html))."] # [cfg (not (target_os = "fuchsia"))] pub fn ttyname < F : std :: os :: fd :: AsFd > (fd : F) -> Result < PathBuf > { use std :: os :: fd :: AsRawFd ; # [cfg (not (target_os = "hurd"))] const PATH_MAX : usize = libc :: PATH_MAX as usize ; # [cfg (target_os = "hurd")] const PATH_MAX : usize = 1024 ; let mut buf = vec ! [0_u8 ; PATH_MAX] ; let c_buf = buf . as_mut_ptr () . cast () ; let ret = unsafe { libc :: ttyname_r (fd . as_fd () . as_raw_fd () , c_buf , buf . len ()) } ; if ret != 0 { return Err (Errno :: from_raw (ret)) ; } CStr :: from_bytes_until_nul (& buf [..]) . map (| s | OsStr :: from_bytes (s . to_bytes ()) . into ()) . map_err (| _ | Errno :: EINVAL) } }
    };
}

macro_239!();