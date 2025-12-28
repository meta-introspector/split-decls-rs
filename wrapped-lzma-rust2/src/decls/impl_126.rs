macro_rules! deps {
    () => {
        WorkerHandle!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < T > WorkerHandle < T > { # [doc = " Attempts to steal work from the queue. Blocks until work is available or the queue is closed."] # [doc = " Returns `None` if the queue is closed and empty."] pub (crate) fn steal (& self) -> Option < T > { let mut queue = self . inner . queue . lock () . unwrap () ; loop { if let Some (item) = queue . pop_front () { return Some (item) ; } if self . inner . closed . load (core :: sync :: atomic :: Ordering :: Acquire) { return None ; } queue = self . inner . condvar . wait (queue) . unwrap () ; } } # [doc = " Attempts to steal work without blocking."] # [doc = " Returns `None` if no work is currently available."] pub (crate) fn try_steal (& self) -> Option < T > { self . inner . queue . lock () . unwrap () . pop_front () } # [doc = " Returns `true` if the queue is closed and empty (no more work will ever be available)."] pub (crate) fn is_closed_and_empty (& self) -> bool { let queue = self . inner . queue . lock () . unwrap () ; let closed = self . inner . closed . load (core :: sync :: atomic :: Ordering :: Acquire) ; closed && queue . is_empty () } }
    };
}

impl_126!();