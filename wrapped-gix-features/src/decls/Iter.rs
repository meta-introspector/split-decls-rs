macro_rules! Iter {
    () => {
        # [doc = " A wrapper for an inner iterator which will check for interruptions on each iteration, stopping the iteration when"] # [doc = " that is requested."] pub struct Iter < 'a , I > { # [doc = " The actual iterator to yield elements from."] pub inner : I , should_interrupt : & 'a AtomicBool , }
    };
}

Iter!();