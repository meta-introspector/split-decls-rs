// Generated macro for impl_147 (impl)
macro_rules! Depcrate_registerimpl_147 {
() => {
// Module: crate::register
// Provides: {"impl_147"}
// Dependencies: {}
impl Restriction { # [doc = " Allow an `io_uring_register` opcode."] pub fn register_op (op : u8) -> Restriction { let mut res = res_zeroed () ; res . opcode = sys :: IORING_RESTRICTION_REGISTER_OP as _ ; res . __bindgen_anon_1 . register_op = op ; Restriction (res) } # [doc = " Allow a submission queue event opcode."] pub fn sqe_op (op : u8) -> Restriction { let mut res = res_zeroed () ; res . opcode = sys :: IORING_RESTRICTION_SQE_OP as _ ; res . __bindgen_anon_1 . sqe_op = op ; Restriction (res) } # [doc = " Allow the given [submission queue event flags](crate::squeue::Flags)."] pub fn sqe_flags_allowed (flags : u8) -> Restriction { let mut res = res_zeroed () ; res . opcode = sys :: IORING_RESTRICTION_SQE_FLAGS_ALLOWED as _ ; res . __bindgen_anon_1 . sqe_flags = flags ; Restriction (res) } # [doc = " Require the given [submission queue event flags](crate::squeue::Flags). These flags must be"] # [doc = " set on every submission."] pub fn sqe_flags_required (flags : u8) -> Restriction { let mut res = res_zeroed () ; res . opcode = sys :: IORING_RESTRICTION_SQE_FLAGS_REQUIRED as _ ; res . __bindgen_anon_1 . sqe_flags = flags ; Restriction (res) } }
};
}
