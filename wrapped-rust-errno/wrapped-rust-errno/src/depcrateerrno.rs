// Generated macro for Errno (struct)
macro_rules! DepcrateErrno {
() => {
// Module: crate
// Provides: {"Errno"}
// Dependencies: {}
# [doc = " Wraps a platform-specific error code."] # [doc = ""] # [doc = " The `Display` instance maps the code to a human-readable string. It"] # [doc = " calls [`strerror_r`][1] under POSIX, and [`FormatMessageW`][2] on"] # [doc = " Windows."] # [doc = ""] # [doc = " [1]: http://pubs.opengroup.org/onlinepubs/009695399/functions/strerror.html"] # [doc = " [2]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms679351%28v=vs.85%29.aspx"] # [derive (Copy , Clone , Eq , Ord , PartialEq , PartialOrd , Hash)] pub struct Errno (pub i32) ;
};
}
