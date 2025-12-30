// Generated macro for sock_nonempty (function)
macro_rules! Depcrate_cqueuesock_nonempty {
() => {
// Module: crate::cqueue
// Provides: {"sock_nonempty"}
// Dependencies: {}
# [doc = " Return whether socket has more data ready to read."] # [doc = ""] # [doc = " This corresponds to the `IORING_CQE_F_SOCK_NONEMPTY` flag, and it signals to"] # [doc = " the consumer that the socket has more data that can be read immediately."] # [doc = ""] # [doc = " The io_uring documentation says recv, recv-multishot, recvmsg, and recvmsg-multishot"] # [doc = " can provide this bit in their respective CQE."] pub fn sock_nonempty (flags : u32) -> bool { flags & sys :: IORING_CQE_F_SOCK_NONEMPTY != 0 }
};
}
