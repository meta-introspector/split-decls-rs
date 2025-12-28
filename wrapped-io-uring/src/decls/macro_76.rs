macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_76 {
    () => {
        deps!();
        opcode ! { # [doc = " Register `nbufs` buffers that each have the length `len` with ids starting from `bid` in the"] # [doc = " group `bgid` that can be used for any request. See"] # [doc = " [`BUFFER_SELECT`](crate::squeue::Flags::BUFFER_SELECT) for more info."] pub struct ProvideBuffers { addr : { * mut u8 } , len : { i32 } , nbufs : { u16 } , bgid : { u16 } , bid : { u16 } ;; } pub const CODE = sys :: IORING_OP_PROVIDE_BUFFERS ; pub fn build (self) -> Entry { let ProvideBuffers { addr , len , nbufs , bgid , bid } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = nbufs as _ ; sqe . __bindgen_anon_2 . addr = addr as _ ; sqe . len = len as _ ; sqe . __bindgen_anon_1 . off = bid as _ ; sqe . __bindgen_anon_4 . buf_group = bgid ; Entry (sqe) } }
    };
}

macro_76!()