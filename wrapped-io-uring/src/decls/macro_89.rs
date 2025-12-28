macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_89 {
    () => {
        deps!();
        opcode ! { # [doc = " Send a message (with data) to a target ring."] pub struct MsgRingData { ring_fd : { impl sealed :: UseFd } , result : { i32 } , user_data : { u64 } , user_flags : { Option < u32 > } , ;; opcode_flags : u32 = 0 } pub const CODE = sys :: IORING_OP_MSG_RING ; pub fn build (self) -> Entry { let MsgRingData { ring_fd , result , user_data , user_flags , opcode_flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . __bindgen_anon_2 . addr = sys :: IORING_MSG_DATA . into () ; sqe . fd = ring_fd ; sqe . len = result as u32 ; sqe . __bindgen_anon_1 . off = user_data ; sqe . __bindgen_anon_3 . msg_ring_flags = opcode_flags ; if let Some (flags) = user_flags { sqe . __bindgen_anon_5 . file_index = flags ; unsafe { sqe . __bindgen_anon_3 . msg_ring_flags |= sys :: IORING_MSG_RING_FLAGS_PASS } ; } Entry (sqe) } }
    };
}

macro_89!();