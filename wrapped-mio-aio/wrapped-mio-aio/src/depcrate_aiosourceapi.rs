// Generated macro for SourceApi (trait)
macro_rules! Depcrate_aioSourceApi {
() => {
// Module: crate::aio
// Provides: {"SourceApi"}
// Dependencies: {}
# [doc = " Common methods supported by all POSIX AIO Mio sources"] pub trait SourceApi { # [doc = " Return type of [`SourceApi::aio_return`]."] type Output ; # [doc = " Read the final result of the operation"] fn aio_return (self : Pin < & mut Self >) -> nix :: Result < Self :: Output > ; # [doc = " Ask the operating system to cancel the operation"] # [doc = ""] # [doc = " Most file systems on most operating systems don't actually support"] # [doc = " cancellation; they'll just return `AIO_NOTCANCELED`."] fn cancel (self : Pin < & mut Self >) -> nix :: Result < aio :: AioCancelStat > ; # [doc = " Retrieve the status of an in-progress or complete operation."] # [doc = ""] # [doc = " Not usually needed, since `mio_aio` always uses kqueue for notification."] fn error (self : Pin < & mut Self >) -> nix :: Result < () > ; # [doc = " Does this operation currently have any in-kernel state?"] fn in_progress (& self) -> bool ; # [doc = " Extra registration method needed by Tokio"] # [cfg (feature = "tokio")] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] fn deregister_raw (& mut self) ; # [doc = " Extra registration method needed by Tokio"] # [cfg (feature = "tokio")] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] fn register_raw (& mut self , kq : RawFd , udata : usize) ; # [doc = " Actually start the I/O operation."] # [doc = ""] # [doc = " After calling this method and until [`SourceApi::aio_return`] returns"] # [doc = " `Ok`, the structure may not be moved in memory."] fn submit (self : Pin < & mut Self >) -> nix :: Result < () > ; }
};
}
