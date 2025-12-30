// Generated macro for Read (trait)
macro_rules! Depcrate_tz_concatenatedRead {
() => {
// Module: crate::tz::concatenated
// Provides: {"Read"}
// Dependencies: {}
# [doc = " A crate-internal trait defining the source of concatenated TZif data."] # [doc = ""] # [doc = " Basically, this just provides a way to read a fixed amount of data at a"] # [doc = " particular offset. This is obviously trivial to implement on `&[u8]` (and"] # [doc = " indeed, we do so for testing), but we use it to abstract over platform"] # [doc = " differences when reading from a `File`."] # [doc = ""] # [doc = " The intent is that on Unix, this will use `pread`, which avoids a file"] # [doc = " seek followed by a `read` call."] pub (crate) trait Read { fn read_exact_at (& self , buf : & mut [u8] , offset : u64) -> Result < () , Error > ; }
};
}
