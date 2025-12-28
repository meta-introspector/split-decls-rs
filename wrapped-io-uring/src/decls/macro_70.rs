macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_70 {
    () => {
        deps!();
        opcode ! { # [doc = " Send a message on a socket, equivalent to `send(2)`."] pub struct Send { fd : { impl sealed :: UseFixed } , buf : { * const u8 } , len : { u32 } , ;; flags : i32 = 0 , # [doc = " Set the destination address, for sending from an unconnected socket."] # [doc = ""] # [doc = " When set, `dest_addr_len` must be set as well."] # [doc = " See also `man 3 io_uring_prep_send_set_addr`."] dest_addr : * const libc :: sockaddr = core :: ptr :: null () , dest_addr_len : libc :: socklen_t = 0 , } pub const CODE = sys :: IORING_OP_SEND ; pub fn build (self) -> Entry { let Send { fd , buf , len , flags , dest_addr , dest_addr_len } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_2 . addr = buf as _ ; sqe . __bindgen_anon_1 . addr2 = dest_addr as _ ; sqe . __bindgen_anon_5 . __bindgen_anon_1 . addr_len = dest_addr_len as _ ; sqe . len = len ; sqe . __bindgen_anon_3 . msg_flags = flags as _ ; Entry (sqe) } }
    };
}

macro_70!();