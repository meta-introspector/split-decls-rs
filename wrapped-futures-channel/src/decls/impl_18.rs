macro_rules! deps {
    () => {
        Node!();
        PopResult!();
        Queue!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < T > Queue < T > { # [doc = " Creates a new queue that is safe to share among multiple producers and"] # [doc = " one consumer."] pub (super) fn new () -> Self { let stub = unsafe { Node :: new (None) } ; Self { head : AtomicPtr :: new (stub) , tail : UnsafeCell :: new (stub) } } # [doc = " Pushes a new value onto this queue."] pub (super) fn push (& self , t : T) { unsafe { let n = Node :: new (Some (t)) ; let prev = self . head . swap (n , Ordering :: AcqRel) ; (* prev) . next . store (n , Ordering :: Release) ; } } # [doc = " Pops some data from this queue."] # [doc = ""] # [doc = " Note that the current implementation means that this function cannot"] # [doc = " return `Option<T>`. It is possible for this queue to be in an"] # [doc = " inconsistent state where many pushes have succeeded and completely"] # [doc = " finished, but pops cannot return `Some(t)`. This inconsistent state"] # [doc = " happens when a pusher is preempted at an inopportune moment."] # [doc = ""] # [doc = " This inconsistent state means that this queue does indeed have data, but"] # [doc = " it does not currently have access to it at this time."] # [doc = ""] # [doc = " This function is unsafe because only one thread can call it at a time."] pub (super) unsafe fn pop (& self) -> PopResult < T > { unsafe { let tail = * self . tail . get () ; let next = (* tail) . next . load (Ordering :: Acquire) ; if ! next . is_null () { * self . tail . get () = next ; assert ! ((* tail) . value . is_none ()) ; assert ! ((* next) . value . is_some ()) ; let ret = (* next) . value . take () . unwrap () ; drop (Box :: from_raw (tail)) ; return Data (ret) ; } if self . head . load (Ordering :: Acquire) == tail { Empty } else { Inconsistent } } } # [doc = " Pop an element similarly to `pop` function, but spin-wait on inconsistent"] # [doc = " queue state instead of returning `Inconsistent`."] # [doc = ""] # [doc = " This function is unsafe because only one thread can call it at a time."] pub (super) unsafe fn pop_spin (& self) -> Option < T > { loop { match unsafe { self . pop () } { Empty => return None , Data (t) => return Some (t) , Inconsistent => { thread :: yield_now () ; } } } } }
    };
}

impl_18!();