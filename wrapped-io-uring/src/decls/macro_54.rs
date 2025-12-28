macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_54 {
    () => {
        deps!();
        opcode ! { # [doc = " Attempt to remove an existing [timeout operation](Timeout)."] pub struct TimeoutRemove { user_data : { u64 } , ;; } pub const CODE = sys :: IORING_OP_TIMEOUT_REMOVE ; pub fn build (self) -> Entry { let TimeoutRemove { user_data } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = - 1 ; sqe . __bindgen_anon_2 . addr = user_data ; Entry (sqe) } }
    };
}

macro_54!()