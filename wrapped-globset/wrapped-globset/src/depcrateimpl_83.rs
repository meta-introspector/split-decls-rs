// Generated macro for impl_83 (impl)
macro_rules! Depcrateimpl_83 {
() => {
// Module: crate
// Provides: {"impl_83"}
// Dependencies: {}
impl < 'a > Candidate < 'a > { # [doc = " Create a new candidate for matching from the given path."] pub fn new < P : AsRef < Path > + ? Sized > (path : & 'a P) -> Candidate < 'a > { Self :: from_cow (Vec :: from_path_lossy (path . as_ref ())) } # [doc = " Create a new candidate for matching from the given path as a sequence"] # [doc = " of bytes."] # [doc = ""] # [doc = " Generally speaking, this routine expects the bytes to be"] # [doc = " _conventionally_ UTF-8. It is legal for the byte sequence to contain"] # [doc = " invalid UTF-8. However, if the bytes are in some other encoding that"] # [doc = " isn't ASCII compatible (for example, UTF-16), then the results of"] # [doc = " matching are unspecified."] pub fn from_bytes < P : AsRef < [u8] > + ? Sized > (path : & 'a P) -> Candidate < 'a > { Self :: from_cow (Cow :: Borrowed (path . as_ref ())) } fn from_cow (path : Cow < 'a , [u8] >) -> Candidate < 'a > { let path = normalize_path (path) ; let basename = file_name (& path) . unwrap_or (Cow :: Borrowed (B (""))) ; let ext = file_name_ext (& basename) . unwrap_or (Cow :: Borrowed (B (""))) ; Candidate { path , basename , ext } } fn path_prefix (& self , max : usize) -> & [u8] { if self . path . len () <= max { & * self . path } else { & self . path [.. max] } } fn path_suffix (& self , max : usize) -> & [u8] { if self . path . len () <= max { & * self . path } else { & self . path [self . path . len () - max ..] } } }
};
}
