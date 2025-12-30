// Generated macro for EventFd (struct)
macro_rules! Depcrate_shims_unix_linux_like_eventfdEventFd {
() => {
// Module: crate::shims::unix::linux_like::eventfd
// Provides: {"EventFd"}
// Dependencies: {}
# [doc = " A kind of file descriptor created by `eventfd`."] # [doc = " The `Event` type isn't currently written to by `eventfd`."] # [doc = " The interface is meant to keep track of objects associated"] # [doc = " with a file descriptor. For more information see the man"] # [doc = " page below:"] # [doc = ""] # [doc = " <https://man.netbsd.org/eventfd.2>"] # [derive (Debug)] struct EventFd { # [doc = " The object contains an unsigned 64-bit integer (uint64_t) counter that is maintained by the"] # [doc = " kernel. This counter is initialized with the value specified in the argument initval."] counter : Cell < u64 > , is_nonblock : bool , clock : RefCell < VClock > , # [doc = " A list of thread ids blocked on eventfd::read."] blocked_read_tid : RefCell < Vec < ThreadId > > , # [doc = " A list of thread ids blocked on eventfd::write."] blocked_write_tid : RefCell < Vec < ThreadId > > , }
};
}
