// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorErrorKind {
() => {
// Module: crate::error
// Provides: {"ErrorKind"}
// Dependencies: {}
# [derive (Debug)] enum ErrorKind { Io (io :: Error) , Num (num :: ParseIntError) , Process (process :: ExitStatus) , Utf8 (str :: Utf8Error) , Other (& 'static str) , }
};
}
