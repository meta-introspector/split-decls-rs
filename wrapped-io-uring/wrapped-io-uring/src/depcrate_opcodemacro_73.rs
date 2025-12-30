// Generated macro for macro_73 (macro)
macro_rules! Depcrate_opcodemacro_73 {
() => {
// Module: crate::opcode
// Provides: {"macro_73"}
// Dependencies: {}
opcode ! { # [doc = " Register a timeout operation."] # [doc = ""] # [doc = " A timeout will trigger a wakeup event on the completion ring for anyone waiting for events."] # [doc = " A timeout condition is met when either the specified timeout expires, or the specified number of events have completed."] # [doc = " Either condition will trigger the event."] # [doc = " The request will complete with `-ETIME` if the timeout got completed through expiration of the timer,"] # [doc = " or 0 if the timeout got completed through requests completing on their own."] # [doc = " If the timeout was cancelled before it expired, the request will complete with `-ECANCELED`."] # [derive (Debug)] pub struct Timeout { timespec : { * const types :: Timespec } , ;; # [doc = " `count` may contain a completion event count."] # [doc = " If [`TimeoutFlags::MULTISHOT`](types::TimeoutFlags::MULTISHOT) is set in `flags`, this is the number of repeats."] # [doc = " A value of 0 means the timeout is indefinite and can only be stopped by a removal request."] count : u32 = 0 , flags : types :: TimeoutFlags = types :: TimeoutFlags :: empty () } pub const CODE = sys :: IORING_OP_TIMEOUT ; pub fn build (self) -> Entry { let Timeout { timespec , count , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = - 1 ; sqe . __bindgen_anon_2 . addr = timespec as _ ; sqe . len = 1 ; sqe . __bindgen_anon_1 . off = count as _ ; sqe . __bindgen_anon_3 . timeout_flags = flags . bits () ; Entry (sqe) } }
};
}
