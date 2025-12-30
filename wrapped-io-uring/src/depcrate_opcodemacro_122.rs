// Generated macro for macro_122 (macro)
macro_rules! Depcrate_opcodemacro_122 {
() => {
// Module: crate::opcode
// Provides: {"macro_122"}
// Dependencies: {}
opcode ! { # [doc = " Issue the equivalent of a `waitid(2)` system call."] # [doc = ""] # [doc = " Available since kernel 6.7."] # [derive (Debug)] pub struct WaitId { idtype : { libc :: idtype_t } , id : { libc :: id_t } , options : { libc :: c_int } , ;; infop : * const libc :: siginfo_t = std :: ptr :: null () , flags : libc :: c_uint = 0 , } pub const CODE = sys :: IORING_OP_WAITID ; pub fn build (self) -> Entry { let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = self . id as _ ; sqe . len = self . idtype as _ ; sqe . __bindgen_anon_3 . waitid_flags = self . flags ; sqe . __bindgen_anon_5 . file_index = self . options as _ ; sqe . __bindgen_anon_1 . addr2 = self . infop as _ ; Entry (sqe) } }
};
}
