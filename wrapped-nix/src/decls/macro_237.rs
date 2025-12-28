macro_rules! deps {
    () => {
        Result!();
        NixPath!();
    };
}

macro_rules! macro_237 {
    () => {
        deps!();
        feature ! { #! [feature = "fs"] # [doc = " Checks the file named by `path` for accessibility according to the flags given by `amode`"] # [doc = " See [access(2)](https://pubs.opengroup.org/onlinepubs/9699919799/functions/access.html)"] pub fn access < P : ? Sized + NixPath > (path : & P , amode : AccessFlags) -> Result < () > { let res = path . with_nix_path (| cstr | unsafe { libc :: access (cstr . as_ptr () , amode . bits ()) }) ?; Errno :: result (res) . map (drop) } # [doc = " Checks the file named by `dirfd` and `path` for accessibility according to"] # [doc = " the flags given by `mode`"] # [doc = ""] # [doc = " # References"] # [doc = ""] # [doc = " [faccessat(2)](http://pubs.opengroup.org/onlinepubs/9699919799/functions/faccessat.html)"] # [cfg (not (target_os = "redox"))] pub fn faccessat < Fd : std :: os :: fd :: AsFd , P : ? Sized + NixPath > (dirfd : Fd , path : & P , mode : AccessFlags , flags : AtFlags ,) -> Result < () > { use std :: os :: fd :: AsRawFd ; let res = path . with_nix_path (| cstr | unsafe { libc :: faccessat (dirfd . as_fd () . as_raw_fd () , cstr . as_ptr () , mode . bits () , flags . bits () ,) }) ?; Errno :: result (res) . map (drop) } # [doc = " Checks the file named by `path` for accessibility according to the flags given"] # [doc = " by `mode` using effective UID, effective GID and supplementary group lists."] # [doc = ""] # [doc = " # References"] # [doc = ""] # [doc = " * [FreeBSD man page](https://www.freebsd.org/cgi/man.cgi?query=eaccess&sektion=2&n=1)"] # [doc = " * [Linux man page](https://man7.org/linux/man-pages/man3/euidaccess.3.html)"] # [cfg (any (freebsdlike , all (target_os = "linux" , not (target_env = "uclibc")) ,))] pub fn eaccess < P : ? Sized + NixPath > (path : & P , mode : AccessFlags) -> Result < () > { let res = path . with_nix_path (| cstr | unsafe { libc :: eaccess (cstr . as_ptr () , mode . bits ()) }) ?; Errno :: result (res) . map (drop) } }
    };
}

macro_237!();