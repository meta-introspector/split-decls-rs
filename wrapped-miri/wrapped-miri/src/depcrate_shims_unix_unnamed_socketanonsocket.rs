// Generated macro for AnonSocket (struct)
macro_rules! Depcrate_shims_unix_unnamed_socketAnonSocket {
() => {
// Module: crate::shims::unix::unnamed_socket
// Provides: {"AnonSocket"}
// Dependencies: {}
# [doc = " One end of a pair of connected unnamed sockets."] # [derive (Debug)] struct AnonSocket { # [doc = " The buffer we are reading from, or `None` if this is the writing end of a pipe."] # [doc = " (In that case, the peer FD will be the reading end of that pipe.)"] readbuf : Option < RefCell < Buffer > > , # [doc = " The `AnonSocket` file descriptor that is our \"peer\", and that holds the buffer we are"] # [doc = " writing to. This is a weak reference because the other side may be closed before us; all"] # [doc = " future writes will then trigger EPIPE."] peer_fd : OnceCell < WeakFileDescriptionRef < AnonSocket > > , # [doc = " Indicates whether the peer has lost data when the file description is closed."] # [doc = " This flag is set to `true` if the peer's `readbuf` is non-empty at the time"] # [doc = " of closure."] peer_lost_data : Cell < bool > , # [doc = " A list of thread ids blocked because the buffer was empty."] # [doc = " Once another thread writes some bytes, these threads will be unblocked."] blocked_read_tid : RefCell < Vec < ThreadId > > , # [doc = " A list of thread ids blocked because the buffer was full."] # [doc = " Once another thread reads some bytes, these threads will be unblocked."] blocked_write_tid : RefCell < Vec < ThreadId > > , # [doc = " Whether this fd is non-blocking or not."] is_nonblock : Cell < bool > , fd_type : AnonSocketType , }
};
}
