// Generated macro for macro_119 (macro)
macro_rules! Depcrate_opcodemacro_119 {
() => {
// Module: crate::opcode
// Provides: {"macro_119"}
// Dependencies: {}
opcode ! { # [doc = " Wait on a futex, like but not equivalant to `futex(2)`'s `FUTEX_WAIT_BITSET`."] # [doc = ""] # [doc = " Wait on a futex at address `futex` and which still has the value `val` and with `futex2(2)`"] # [doc = " flags of `futex_flags`. `musk` can be set to a specific bitset mask, which will be matched"] # [doc = " by the waking side to decide who to wake up. To always get woken, an application may use"] # [doc = " `FUTEX_BITSET_MATCH_ANY` (truncated to futex bits). `futex_flags` follows the `futex2(2)`"] # [doc = " flags, not the `futex(2)` v1 interface flags. `flags` are currently unused and hence `0`"] # [doc = " must be passed."] # [derive (Debug)] pub struct FutexWait { futex : { * const u32 } , val : { u64 } , mask : { u64 } , futex_flags : { u32 } , ;; flags : u32 = 0 } pub const CODE = sys :: IORING_OP_FUTEX_WAIT ; pub fn build (self) -> Entry { let FutexWait { futex , val , mask , futex_flags , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = futex_flags as _ ; sqe . __bindgen_anon_2 . addr = futex as usize as _ ; sqe . __bindgen_anon_1 . off = val ; unsafe { sqe . __bindgen_anon_6 . __bindgen_anon_1 . as_mut () . addr3 = mask } ; sqe . __bindgen_anon_3 . futex_flags = flags ; Entry (sqe) } }
};
}
