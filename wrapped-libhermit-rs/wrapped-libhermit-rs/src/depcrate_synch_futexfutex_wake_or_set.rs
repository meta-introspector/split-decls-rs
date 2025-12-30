// Generated macro for futex_wake_or_set (function)
macro_rules! Depcrate_synch_futexfutex_wake_or_set {
() => {
// Module: crate::synch::futex
// Provides: {"futex_wake_or_set"}
// Dependencies: {}
# [doc = " Wake `count` threads waiting on the futex at address. Returns the number of threads"] # [doc = " woken up (saturates to `i32::MAX`). If `count` is `i32::MAX`, wake up all matching"] # [doc = " waiting threads. If `count` is negative, returns -EINVAL. If no thread is available,"] # [doc = " the futex at address will set to `new_value`."] pub (crate) fn futex_wake_or_set (address : & AtomicU32 , count : i32 , new_value : u32) -> i32 { if count < 0 { return - i32 :: from (Errno :: Inval) ; } let mut parking_lot = PARKING_LOT . lock () ; let mut queue = match parking_lot . entry (addr (address)) { Entry :: Occupied (entry) => entry , Entry :: Vacant (_) => { address . store (new_value , SeqCst) ; return 0 ; } } ; let scheduler = core_scheduler () ; let mut woken = 0 ; while woken != count || count == i32 :: MAX { match queue . get_mut () . pop () { Some (handle) => scheduler . custom_wakeup (handle) , None => break , } woken = woken . saturating_add (1) ; } if queue . get () . is_empty () { queue . remove () ; } if woken == 0 { address . store (new_value , SeqCst) ; } woken }
};
}
