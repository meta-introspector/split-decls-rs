// Generated macro for EpollEvents (struct)
macro_rules! Depcrate_shims_unix_linux_like_epollEpollEvents {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"EpollEvents"}
// Dependencies: {}
# [doc = " EpollReadyEvents reflects the readiness of a file description."] # [derive (Debug)] pub struct EpollEvents { # [doc = " The associated file is available for read(2) operations, in the sense that a read will not block."] # [doc = " (I.e., returning EOF is considered \"ready\".)"] pub epollin : bool , # [doc = " The associated file is available for write(2) operations, in the sense that a write will not block."] pub epollout : bool , # [doc = " Stream socket peer closed connection, or shut down writing"] # [doc = " half of connection."] pub epollrdhup : bool , # [doc = " For stream socket, this event merely indicates that the peer"] # [doc = " closed its end of the channel."] # [doc = " Unlike epollrdhup, this should only be set when the stream is fully closed."] # [doc = " epollrdhup also gets set when only the write half is closed, which is possible"] # [doc = " via `shutdown(_, SHUT_WR)`."] pub epollhup : bool , # [doc = " Error condition happened on the associated file descriptor."] pub epollerr : bool , }
};
}
