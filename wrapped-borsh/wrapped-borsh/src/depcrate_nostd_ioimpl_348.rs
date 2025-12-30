// Generated macro for impl_348 (impl)
macro_rules! Depcrate_nostd_ioimpl_348 {
() => {
// Module: crate::nostd_io
// Provides: {"impl_348"}
// Dependencies: {}
# [doc = " Intended for use for errors not exposed to the user, where allocating onto"] # [doc = " the heap (for normal construction via Error::new) is too costly."] impl From < ErrorKind > for Error { # [doc = " Converts an [`ErrorKind`] into an [`Error`]."] # [doc = ""] # [doc = " This conversion allocates a new error with a simple representation of error kind."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::{Error, ErrorKind};"] # [doc = ""] # [doc = " let not_found = ErrorKind::NotFound;"] # [doc = " let error = Error::from(not_found);"] # [doc = " assert_eq!(\"entity not found\", format!(\"{}\", error));"] # [doc = " ```"] # [inline] fn from (kind : ErrorKind) -> Error { Error { repr : Repr :: Simple (kind) , } } }
};
}
