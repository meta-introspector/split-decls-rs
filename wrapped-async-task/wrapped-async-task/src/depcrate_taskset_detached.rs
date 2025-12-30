// Generated macro for set_detached (function)
macro_rules! Depcrate_taskset_detached {
() => {
// Module: crate::task
// Provides: {"set_detached"}
// Dependencies: {}
# [doc = " Puts the task in detached state."] # [inline (never)] fn set_detached < T > (ptr : * const ()) -> Option < Result < T , Panic > > { let header = ptr as * const Header ; unsafe { let mut output = None ; if let Err (mut state) = (* header) . state . compare_exchange_weak (SCHEDULED | TASK | REFERENCE , SCHEDULED | REFERENCE , Ordering :: AcqRel , Ordering :: Acquire ,) { loop { if state & COMPLETED != 0 && state & CLOSED == 0 { match (* header) . state . compare_exchange_weak (state , state | CLOSED , Ordering :: AcqRel , Ordering :: Acquire ,) { Ok (_) => { output = Some (((* header) . vtable . get_output (ptr) as * mut Result < T , Panic >) . read () ,) ; state |= CLOSED ; } Err (s) => state = s , } } else { let new = if state & (! (REFERENCE - 1) | CLOSED) == 0 { SCHEDULED | CLOSED | REFERENCE } else { state & ! TASK } ; match (* header) . state . compare_exchange_weak (state , new , Ordering :: AcqRel , Ordering :: Acquire ,) { Ok (_) => { if state & ! (REFERENCE - 1) == 0 { if state & CLOSED == 0 { ((* header) . vtable . schedule) (ptr , ScheduleInfo :: new (false)) ; } else { ((* header) . vtable . destroy) (ptr) ; } } break ; } Err (s) => state = s , } } } } output } }
};
}
