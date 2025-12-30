// Generated macro for Error (struct)
macro_rules! Depcrate_core_errorError {
() => {
// Module: crate::core::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error produced by recursively walking a directory."] # [doc = ""] # [doc = " This error type is a light wrapper around [`std::io::Error`]. In"] # [doc = " particular, it adds the following information:"] # [doc = ""] # [doc = " * The depth at which the error occurred in the file tree, relative to the"] # [doc = " root."] # [doc = " * The path, if any, associated with the IO error."] # [doc = " * An indication that a loop occurred when following symbolic links. In this"] # [doc = " case, there is no underlying IO error."] # [doc = ""] # [doc = " To maintain good ergonomics, this type has a"] # [doc = " [`impl From<Error> for std::io::Error`][impl] defined which preserves the original context."] # [doc = " This allows you to use an [`io::Result`] with methods in this crate if you don't care about"] # [doc = " accessing the underlying error data in a structured form."] # [doc = ""] # [doc = " [`std::io::Error`]: https://doc.rust-lang.org/stable/std/io/struct.Error.html"] # [doc = " [`io::Result`]: https://doc.rust-lang.org/stable/std/io/type.Result.html"] # [doc = " [impl]: struct.Error.html#impl-From%3CError%3E"] # [derive (Debug)] pub struct Error { depth : usize , inner : ErrorInner , }
};
}
