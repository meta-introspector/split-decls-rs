macro_rules! deps {
    () => {
        Entry!();
        FutexWaitV!();
    };
}

macro_rules! macro_101 {
    () => {
        deps!();
        opcode ! { # [doc = " Wait on multiple futexes."] # [doc = ""] # [doc = " Wait on multiple futexes at the same time. Futexes are given by `futexv` and `nr_futex` is"] # [doc = " the number of futexes in that array. Unlike `FutexWait`, the desired bitset mask and values"] # [doc = " are passed in `futexv`. `flags` are currently unused and hence `0` must be passed."] # [derive (Debug)] pub struct FutexWaitV { futexv : { * const types :: FutexWaitV } , nr_futex : { u32 } , ;; flags : u32 = 0 } pub const CODE = sys :: IORING_OP_FUTEX_WAITV ; pub fn build (self) -> Entry { let FutexWaitV { futexv , nr_futex , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . __bindgen_anon_2 . addr = futexv as usize as _ ; sqe . len = nr_futex ; sqe . __bindgen_anon_3 . futex_flags = flags ; Entry (sqe) } }
    };
}

macro_101!()