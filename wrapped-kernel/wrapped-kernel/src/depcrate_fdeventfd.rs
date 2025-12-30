// Generated macro for eventfd (function)
macro_rules! Depcrate_fdeventfd {
() => {
// Module: crate::fd
// Provides: {"eventfd"}
// Dependencies: {}
# [doc = " Wait for some event on a file descriptor."] # [doc = ""] # [doc = " `eventfd` creates an linux-like \"eventfd object\" that can be used"] # [doc = " as an event wait/notify mechanism by user-space applications, and by"] # [doc = " the kernel to notify user-space applications of events. The"] # [doc = " object contains an unsigned 64-bit integer counter"] # [doc = " that is maintained by the kernel. This counter is initialized"] # [doc = " with the value specified in the argument `initval`."] # [doc = ""] # [doc = " As its return value, `eventfd` returns a new file descriptor that"] # [doc = " can be used to refer to the eventfd object."] # [doc = ""] # [doc = " The following values may be bitwise set in flags to change the"] # [doc = " behavior of `eventfd`:"] # [doc = ""] # [doc = " `EFD_NONBLOCK`: Set the file descriptor in non-blocking mode"] # [doc = " `EFD_SEMAPHORE`: Provide semaphore-like semantics for reads"] # [doc = " from the new file descriptor."] pub fn eventfd (initval : u64 , flags : EventFlags) -> io :: Result < FileDescriptor > { let obj = self :: eventfd :: EventFd :: new (initval , flags) ; let fd = core_scheduler () . insert_object (Arc :: new (async_lock :: RwLock :: new (obj))) ? ; Ok (fd) }
};
}
