// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorErrorKind {
() => {
// Module: crate::error
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " The kind of error."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum ErrorKind { # [doc = " A parse error occurred while reading the file."] Parse , # [doc = " A validation error occurred while writing the file."] Write , # [doc = " An I/O error occurred while writing the file."] Io (io :: ErrorKind) , # [doc = " A validation error occurred while modifying the file."] Modify , }
};
}
