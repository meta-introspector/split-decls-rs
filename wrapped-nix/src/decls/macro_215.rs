macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! macro_215 {
    () => {
        deps!();
        feature ! { #! [feature = "hostname"] # [doc = " Set the system host name (see"] # [doc = " [sethostname(2)](https://man7.org/linux/man-pages/man2/gethostname.2.html))."] # [doc = ""] # [doc = " Given a name, attempt to update the system host name to the given string."] # [doc = " On some systems, the host name is limited to as few as 64 bytes.  An error"] # [doc = " will be returned if the name is not valid or the current process does not"] # [doc = " have permissions to update the host name."] # [cfg (not (target_os = "redox"))] pub fn sethostname < S : AsRef < OsStr >> (name : S) -> Result < () > { cfg_if ! { if # [cfg (any (freebsdlike , solarish , apple_targets , target_os = "aix"))] { type sethostname_len_t = c_int ; } else { type sethostname_len_t = size_t ; } } let ptr = name . as_ref () . as_bytes () . as_ptr () . cast () ; let len = name . as_ref () . len () as sethostname_len_t ; let res = unsafe { libc :: sethostname (ptr , len) } ; Errno :: result (res) . map (drop) } # [doc = " Get the host name and store it in an internally allocated buffer, returning an"] # [doc = " `OsString` on success."] # [doc = ""] # [doc = " This function call attempts to get the host name for the running system and"] # [doc = " store it in an internal buffer, returning it as an `OsString` if successful."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use nix::unistd;"] # [doc = ""] # [doc = " let hostname = unistd::gethostname().expect(\"Failed getting hostname\");"] # [doc = " let hostname = hostname.into_string().expect(\"Hostname wasn't valid UTF-8\");"] # [doc = " println!(\"Hostname: {}\", hostname);"] # [doc = " ```"] # [doc = ""] # [doc = " See also [gethostname(2)](https://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostname.html)."] pub fn gethostname () -> Result < OsString > { let mut buffer : Vec < u8 > = Vec :: with_capacity (256) ; let ptr = buffer . as_mut_ptr () . cast () ; let len = buffer . capacity () as size_t ; let res = unsafe { libc :: gethostname (ptr , len) } ; Errno :: result (res) . map (| _ | { unsafe { buffer . as_mut_ptr () . wrapping_add (len - 1) . write (0) ; let len = CStr :: from_ptr (buffer . as_ptr () . cast ()) . len () ; buffer . set_len (len) ; } OsString :: from_vec (buffer) }) } }
    };
}

macro_215!();