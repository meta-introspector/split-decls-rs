macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_48 {
    () => {
        deps!();
        opcode ! { # [doc = " Remove an existing [poll](PollAdd) request."] # [doc = ""] # [doc = " If found, the `result` method of the `cqueue::Entry` will return 0."] # [doc = " If not found, `result` will return `-libc::ENOENT`."] # [derive (Debug)] pub struct PollRemove { user_data : { u64 } ;; } pub const CODE = sys :: IORING_OP_POLL_REMOVE ; pub fn build (self) -> Entry { let PollRemove { user_data } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = - 1 ; sqe . __bindgen_anon_2 . addr = user_data ; Entry (sqe) } }
    };
}

macro_48!();