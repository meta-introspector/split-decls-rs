// Generated macro for Error (enum)
macro_rules! Depcrate_unicodeError {
() => {
// Module: crate::unicode
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error that occurs when dealing with Unicode."] # [doc = ""] # [doc = " We don't impl the Error trait here because these always get converted"] # [doc = " into other public errors. (This error type isn't exported.)"] # [derive (Debug)] pub enum Error { PropertyNotFound , PropertyValueNotFound , # [allow (dead_code)] PerlClassNotFound , }
};
}
