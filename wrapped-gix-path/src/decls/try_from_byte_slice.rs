macro_rules! deps {
    () => {
        Utf8Error!();
    };
}

macro_rules! try_from_byte_slice {
    () => {
        deps!();
        # [doc = " Given `input` bytes, produce a `Path` from them ignoring encoding entirely if on Unix."] # [doc = ""] # [doc = " On Windows, the input is required to be valid UTF-8, which is guaranteed if we wrote it before."] # [doc = " There are some potential Git versions and Windows installations which produce malformed UTF-16"] # [doc = " if certain emojis are in the path. It's as rare as it sounds, but possible."] pub fn try_from_byte_slice (input : & [u8]) -> Result < & Path , Utf8Error > { # [cfg (unix)] let p = { use std :: os :: unix :: ffi :: OsStrExt ; OsStr :: from_bytes (input) . as_ref () } ; # [cfg (target_os = "wasi")] let p : & Path = { use std :: os :: wasi :: ffi :: OsStrExt ; OsStr :: from_bytes (input) . as_ref () } ; # [cfg (not (any (unix , target_os = "wasi")))] let p = Path :: new (std :: str :: from_utf8 (input) . map_err (| _ | Utf8Error) ?) ; Ok (p) }
    };
}

try_from_byte_slice!();