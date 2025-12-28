macro_rules! deps {
    () => {
        UnparkToken!();
        UnparkResult!();
    };
}

macro_rules! unpark_one {
    () => {
        deps!();
        # [doc = " Unparks one thread from the queue associated with the given key."] # [doc = ""] # [doc = " The `callback` function is called while the queue is locked and before the"] # [doc = " target thread is woken up. The `UnparkResult` argument to the function"] # [doc = " indicates whether a thread was found in the queue and whether this was the"] # [doc = " last thread in the queue. This value is also returned by `unpark_one`."] # [doc = ""] # [doc = " The `callback` function should return an `UnparkToken` value which will be"] # [doc = " passed to the thread that is unparked. If no thread is unparked then the"] # [doc = " returned value is ignored."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " You should only call this function with an address that you control, since"] # [doc = " you could otherwise interfere with the operation of other synchronization"] # [doc = " primitives."] # [doc = ""] # [doc = " The `callback` function is called while the queue is locked and must not"] # [doc = " panic or call into any function in `parking_lot`."] # [doc = ""] # [doc = " The `parking_lot` functions are not re-entrant and calling this method"] # [doc = " from the context of an asynchronous signal handler may result in undefined"] # [doc = " behavior, including corruption of internal state and/or deadlocks."] # [inline] pub unsafe fn unpark_one (key : usize , callback : impl FnOnce (UnparkResult) -> UnparkToken ,) -> UnparkResult { let bucket = lock_bucket (key) ; let mut link = & bucket . queue_head ; let mut current = bucket . queue_head . get () ; let mut previous = ptr :: null () ; let mut result = UnparkResult :: default () ; while ! current . is_null () { if (* current) . key . load (Ordering :: Relaxed) == key { let next = (* current) . next_in_queue . get () ; link . set (next) ; if bucket . queue_tail . get () == current { bucket . queue_tail . set (previous) ; } else { let mut scan = next ; while ! scan . is_null () { if (* scan) . key . load (Ordering :: Relaxed) == key { result . have_more_threads = true ; break ; } scan = (* scan) . next_in_queue . get () ; } } result . unparked_threads = 1 ; result . be_fair = (* bucket . fair_timeout . get ()) . should_timeout () ; let token = callback (result) ; (* current) . unpark_token . set (token) ; let handle = (* current) . parker . unpark_lock () ; bucket . mutex . unlock () ; handle . unpark () ; return result ; } else { link = & (* current) . next_in_queue ; previous = current ; current = link . get () ; } } callback (result) ; bucket . mutex . unlock () ; result }
    };
}

unpark_one!();