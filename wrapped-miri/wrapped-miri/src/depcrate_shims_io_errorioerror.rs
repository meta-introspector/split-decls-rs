// Generated macro for IoError (enum)
macro_rules! Depcrate_shims_io_errorIoError {
() => {
// Module: crate::shims::io_error
// Provides: {"IoError"}
// Dependencies: {}
# [doc = " A representation of an IO error: either a libc error name,"] # [doc = " or a host error."] # [derive (Debug)] pub enum IoError { LibcError (& 'static str) , WindowsError (& 'static str) , HostError (io :: Error) , Raw (Scalar) , }
};
}
