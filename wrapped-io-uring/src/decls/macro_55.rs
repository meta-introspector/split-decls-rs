macro_rules! deps {
    () => {
        Timespec!();
        Entry!();
    };
}

macro_rules! macro_55 {
    () => {
        deps!();
        opcode ! { # [doc = " Attempt to update an existing [timeout operation](Timeout) with a new timespec."] # [doc = " The optional `count` value of the original timeout value cannot be updated."] pub struct TimeoutUpdate { user_data : { u64 } , timespec : { * const types :: Timespec } , ;; flags : types :: TimeoutFlags = types :: TimeoutFlags :: empty () } pub const CODE = sys :: IORING_OP_TIMEOUT_REMOVE ; pub fn build (self) -> Entry { let TimeoutUpdate { user_data , timespec , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = - 1 ; sqe . __bindgen_anon_1 . off = timespec as _ ; sqe . __bindgen_anon_2 . addr = user_data ; sqe . __bindgen_anon_3 . timeout_flags = flags . bits () | sys :: IORING_TIMEOUT_UPDATE ; Entry (sqe) } }
    };
}

macro_55!();