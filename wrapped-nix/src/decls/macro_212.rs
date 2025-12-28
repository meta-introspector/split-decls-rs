macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! macro_212 {
    () => {
        deps!();
        feature ! { #! [feature = "fs"] # [doc = " Returns the current directory as a `PathBuf`"] # [doc = ""] # [doc = " Err is returned if the current user doesn't have the permission to read or search a component"] # [doc = " of the current path."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use nix::unistd;"] # [doc = ""] # [doc = " // assume that we are allowed to get current directory"] # [doc = " let dir = unistd::getcwd().unwrap();"] # [doc = " println!(\"The current directory is {:?}\", dir);"] # [doc = " ```"] # [inline] pub fn getcwd () -> Result < PathBuf > { let mut buf = Vec ::< u8 >:: with_capacity (512) ; loop { unsafe { let ptr = buf . as_mut_ptr () . cast () ; if ! libc :: getcwd (ptr , buf . capacity ()) . is_null () { let len = CStr :: from_ptr (buf . as_ptr () . cast ()) . to_bytes () . len () ; buf . set_len (len) ; buf . shrink_to_fit () ; return Ok (PathBuf :: from (OsString :: from_vec (buf))) ; } else { let error = Errno :: last () ; if error != Errno :: ERANGE { return Err (error) ; } } # [cfg (not (target_os = "hurd"))] const PATH_MAX : usize = libc :: PATH_MAX as usize ; # [cfg (target_os = "hurd")] const PATH_MAX : usize = 1024 ; reserve_double_buffer_size (& mut buf , PATH_MAX) ?; } } } }
    };
}

macro_212!()