// Generated macro for AllowStdIo (struct)
macro_rules! Depcrate_io_allow_stdAllowStdIo {
() => {
// Module: crate::io::allow_std
// Provides: {"AllowStdIo"}
// Dependencies: {}
# [doc = " A simple wrapper type which allows types which implement only"] # [doc = " implement `std::io::Read` or `std::io::Write`"] # [doc = " to be used in contexts which expect an `AsyncRead` or `AsyncWrite`."] # [doc = ""] # [doc = " If these types issue an error with the kind `io::ErrorKind::WouldBlock`,"] # [doc = " it is expected that they will notify the current task on readiness."] # [doc = " Synchronous `std` types should not issue errors of this kind and"] # [doc = " are safe to use in this context. However, using these types with"] # [doc = " `AllowStdIo` will cause the event loop to block, so they should be used"] # [doc = " with care."] # [derive (Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct AllowStdIo < T > (T) ;
};
}
