macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_100 {
    () => {
        deps!();
        opcode ! { # [doc = " Wake up waiters on a futex, like but not equivalant to `futex(2)`'s `FUTEX_WAKE_BITSET`."] # [doc = ""] # [doc = " Wake any waiters on the futex indicated by `futex` and at most `val` futexes. `futex_flags`"] # [doc = " indicates the `futex2(2)` modifier flags. If a given bitset for who to wake is desired,"] # [doc = " then that must be set in `mask`. Use `FUTEX_BITSET_MATCH_ANY` (truncated to futex bits) to"] # [doc = " match any waiter on the given futex. `flags` are currently unused and hence `0` must be"] # [doc = " passed."] # [derive (Debug)] pub struct FutexWake { futex : { * const u32 } , val : { u64 } , mask : { u64 } , futex_flags : { u32 } , ;; flags : u32 = 0 } pub const CODE = sys :: IORING_OP_FUTEX_WAKE ; pub fn build (self) -> Entry { let FutexWake { futex , val , mask , futex_flags , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = futex_flags as _ ; sqe . __bindgen_anon_2 . addr = futex as usize as _ ; sqe . __bindgen_anon_1 . off = val ; unsafe { sqe . __bindgen_anon_6 . __bindgen_anon_1 . as_mut () . addr3 = mask } ; sqe . __bindgen_anon_3 . futex_flags = flags ; Entry (sqe) } }
    };
}

macro_100!()