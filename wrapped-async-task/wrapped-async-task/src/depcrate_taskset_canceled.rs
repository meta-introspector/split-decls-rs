// Generated macro for set_canceled (function)
macro_rules! Depcrate_taskset_canceled {
() => {
// Module: crate::task
// Provides: {"set_canceled"}
// Dependencies: {}
# [doc = " Puts the task in canceled state."] # [inline (never)] fn set_canceled (ptr : * const ()) { let header = ptr as * const Header ; unsafe { let mut state = (* header) . state . load (Ordering :: Acquire) ; loop { if state & (COMPLETED | CLOSED) != 0 { break ; } let new = if state & (SCHEDULED | RUNNING) == 0 { (state | SCHEDULED | CLOSED) + REFERENCE } else { state | CLOSED } ; match (* header) . state . compare_exchange_weak (state , new , Ordering :: AcqRel , Ordering :: Acquire ,) { Ok (_) => { if state & (SCHEDULED | RUNNING) == 0 { ((* header) . vtable . schedule) (ptr , ScheduleInfo :: new (false)) ; } if state & AWAITER != 0 { (* header) . notify (None) ; } break ; } Err (s) => state = s , } } } }
};
}
