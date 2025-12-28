macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_77 {
    () => {
        deps!();
        opcode ! { # [doc = " Remove some number of buffers from a buffer group. See"] # [doc = " [`BUFFER_SELECT`](crate::squeue::Flags::BUFFER_SELECT) for more info."] pub struct RemoveBuffers { nbufs : { u16 } , bgid : { u16 } ;; } pub const CODE = sys :: IORING_OP_REMOVE_BUFFERS ; pub fn build (self) -> Entry { let RemoveBuffers { nbufs , bgid } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = nbufs as _ ; sqe . __bindgen_anon_4 . buf_group = bgid ; Entry (sqe) } }
    };
}

macro_77!()