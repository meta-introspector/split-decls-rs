// Generated macro for Error (struct)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " A small and `no_std` compatible error type"] # [doc = ""] # [doc = " The [`Error::raw_os_error()`] will indicate if the error is from the OS, and"] # [doc = " if so, which error code the OS gave the application. If such an error is"] # [doc = " encountered, please consult with your system documentation."] # [doc = ""] # [doc = " *If this crate's `\"std\"` Cargo feature is enabled*, then:"] # [doc = " - [`getrandom::Error`][Error] implements"] # [doc = "   [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)"] # [doc = " - [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) implements"] # [doc = "   [`From<getrandom::Error>`](https://doc.rust-lang.org/std/convert/trait.From.html)."] # [derive (Copy , Clone , Eq , PartialEq)] pub struct Error (NonZeroRawOsError) ;
};
}
