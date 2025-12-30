// Generated macro for wake_by_ref (function)
macro_rules! Depcrate_rawwake_by_ref {
() => {
// Module: crate::raw
// Provides: {"wake_by_ref"}
// Dependencies: {}
# [doc = " Wakes a waker by reference."] unsafe fn wake_by_ref < S : Schedule < M > , M > (ptr : * const ()) { let header = ptr as * const Header ; let header = & * header ; let task_layout = header . vtable . layout_info ; let mut state = header . state . load (Ordering :: Acquire) ; loop { if state & (COMPLETED | CLOSED) != 0 { break ; } if state & SCHEDULED != 0 { match header . state . compare_exchange_weak (state , state , Ordering :: AcqRel , Ordering :: Acquire ,) { Ok (_) => break , Err (s) => state = s , } } else { let new = if state & RUNNING == 0 { (state | SCHEDULED) + REFERENCE } else { state | SCHEDULED } ; match header . state . compare_exchange_weak (state , new , Ordering :: AcqRel , Ordering :: Acquire ,) { Ok (_) => { if state & RUNNING == 0 { if state > isize :: MAX as usize { abort () ; } let schedule = ptr . add_byte (task_layout . offset_s) as * mut S ; let task = Runnable :: from_raw (NonNull :: new_unchecked (ptr as * mut ())) ; (* schedule) . schedule (task , ScheduleInfo :: new (false)) ; } break ; } Err (s) => state = s , } } } }
};
}
