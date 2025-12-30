// Generated macro for wake (function)
macro_rules! Depcrate_rawwake {
() => {
// Module: crate::raw
// Provides: {"wake"}
// Dependencies: {}
# [doc = " Wakes a waker."] unsafe fn wake < S : Schedule < M > , M > (ptr : * const ()) { if mem :: size_of :: < S > () > 0 { wake_by_ref :: < S , M > (ptr) ; drop_waker (ptr) ; return ; } let header = ptr as * const Header ; let mut state = (* header) . state . load (Ordering :: Acquire) ; loop { if state & (COMPLETED | CLOSED) != 0 { drop_waker (ptr) ; break ; } if state & SCHEDULED != 0 { match (* header) . state . compare_exchange_weak (state , state , Ordering :: AcqRel , Ordering :: Acquire ,) { Ok (_) => { drop_waker (ptr) ; break ; } Err (s) => state = s , } } else { match (* header) . state . compare_exchange_weak (state , state | SCHEDULED , Ordering :: AcqRel , Ordering :: Acquire ,) { Ok (_) => { if state & RUNNING == 0 { schedule :: < S , M > (ptr , ScheduleInfo :: new (false)) ; } else { drop_waker (ptr) ; } break ; } Err (s) => state = s , } } } }
};
}
