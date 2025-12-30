// Generated macro for impl_54 (impl)
macro_rules! Depcrate_open_optionsimpl_54 {
() => {
// Module: crate::open_options
// Provides: {"impl_54"}
// Dependencies: {}
# [doc = " Methods added by fs-err that are not available on"] # [doc = " [`std::fs::OpenOptions`](https://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html)."] impl OpenOptions { # [doc = " Constructs `Self` from [`std::fs::OpenOptions`](https://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html)"] pub fn from_options (options : fs :: OpenOptions) -> Self { Self (options) } # [doc = " Returns a reference to the underlying [`std::fs::OpenOptions`](https://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html)."] # [doc = ""] # [doc = " Note that calling `open()` on this reference will NOT give you the improved errors from fs-err."] pub fn options (& self) -> & fs :: OpenOptions { & self . 0 } # [doc = " Returns a mutable reference to the underlying [`std::fs::OpenOptions`](https://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html)."] # [doc = ""] # [doc = " This allows you to change settings that don't yet have wrappers in fs-err."] # [doc = " Note that calling `open()` on this reference will NOT give you the improved errors from fs-err."] pub fn options_mut (& mut self) -> & mut fs :: OpenOptions { & mut self . 0 } }
};
}
