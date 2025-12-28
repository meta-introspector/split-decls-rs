macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_47 {
    () => {
        deps!();
        opcode ! { # [doc = " Poll the specified fd."] # [doc = ""] # [doc = " Unlike poll or epoll without `EPOLLONESHOT`, this interface defaults to work in one shot mode."] # [doc = " That is, once the poll operation is completed, it will have to be resubmitted."] # [doc = ""] # [doc = " If multi is set, the poll will work in multi shot mode instead. That means it will"] # [doc = " repeatedly trigger when the requested event becomes true, and hence multiple CQEs can be"] # [doc = " generated from this single submission. The CQE flags field will have IORING_CQE_F_MORE set"] # [doc = " on completion if the application should expect further CQE entries from the original"] # [doc = " request. If this flag isn't set on completion, then the poll request has been terminated"] # [doc = " and no further events will be generated. This mode is available since 5.13."] # [derive (Debug)] pub struct PollAdd { # [doc = " The bits that may be set in `flags` are defined in `<poll.h>`,"] # [doc = " and documented in `poll(2)`."] fd : { impl sealed :: UseFixed } , flags : { u32 } , ;; multi : bool = false } pub const CODE = sys :: IORING_OP_POLL_ADD ; pub fn build (self) -> Entry { let PollAdd { fd , flags , multi } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; if multi { sqe . len = sys :: IORING_POLL_ADD_MULTI ; } # [cfg (target_endian = "little")] { sqe . __bindgen_anon_3 . poll32_events = flags ; } # [cfg (target_endian = "big")] { let x = flags << 16 ; let y = flags >> 16 ; let flags = x | y ; sqe . __bindgen_anon_3 . poll32_events = flags ; } Entry (sqe) } }
    };
}

macro_47!()