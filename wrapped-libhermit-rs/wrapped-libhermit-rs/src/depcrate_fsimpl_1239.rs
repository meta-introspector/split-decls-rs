// Generated macro for impl_1239 (impl)
macro_rules! Depcrate_fsimpl_1239 {
() => {
// Module: crate::fs
// Provides: {"impl_1239"}
// Dependencies: {}
impl File { # [doc = " Creates a new file in read-write mode; error if the file exists."] # [doc = ""] # [doc = " This function will create a file if it does not exist, or return"] # [doc = " an error if it does. This way, if the call succeeds, the file"] # [doc = " returned is guaranteed to be new."] pub fn create (path : & str) -> io :: Result < Self > { let fd = open (path , OpenOption :: O_CREAT | OpenOption :: O_RDWR , AccessPermission :: from_bits (0o666) . unwrap () ,) ? ; Ok (File { fd , path : path . to_string () , }) } # [doc = " Attempts to open a file in read-write mode."] pub fn open (path : & str) -> io :: Result < Self > { let fd = open (path , OpenOption :: O_RDWR , AccessPermission :: from_bits (0o666) . unwrap () ,) ? ; Ok (File { fd , path : path . to_string () , }) } pub fn metadata (& self) -> io :: Result < Metadata > { metadata (& self . path) } }
};
}
