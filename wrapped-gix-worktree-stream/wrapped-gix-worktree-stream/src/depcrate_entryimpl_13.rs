// Generated macro for impl_13 (impl)
macro_rules! Depcrate_entryimpl_13 {
() => {
// Module: crate::entry
// Provides: {"impl_13"}
// Dependencies: {}
impl Entry < '_ > { # [doc = " Return the path of this entry as slash-separated path relative to the repository."] pub fn relative_path (& self) -> & BStr { self . path_buf . as_ref () . expect ("always set during our lifetime") . as_ref () } # [doc = " The amount of bytes that remain to be read, or `None` if it's fully streamed."] # [doc = ""] # [doc = " This equals the length of the entry in bytes right before reading it."] pub fn bytes_remaining (& self) -> Option < usize > { self . remaining } }
};
}
