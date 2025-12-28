macro_rules! deps {
    () => {
        Inner!();
        WorkStealingQueue!();
        WorkerHandle!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < T > WorkStealingQueue < T > { # [doc = " Creates a new work-stealing queue."] pub (crate) fn new () -> Self { Self { inner : Arc :: new (Inner { queue : Mutex :: new (VecDeque :: new ()) , condvar : Condvar :: new () , closed : AtomicBool :: new (false) , }) , } } # [doc = " Creates a worker handle that can steal work from this queue."] pub (crate) fn worker (& self) -> WorkerHandle < T > { WorkerHandle { inner : Arc :: clone (& self . inner) , } } # [doc = " Pushes work to the queue. Returns false if the queue is closed."] pub (crate) fn push (& self , item : T) -> bool { if self . inner . closed . load (core :: sync :: atomic :: Ordering :: Acquire) { return false ; } { let mut queue = self . inner . queue . lock () . unwrap () ; queue . push_back (item) ; } self . inner . condvar . notify_one () ; true } # [doc = " Closes the queue, preventing new work from being added."] # [doc = " Workers will continue to process remaining work until the queue is empty."] pub (crate) fn close (& self) { self . inner . closed . store (true , core :: sync :: atomic :: Ordering :: Release) ; self . inner . condvar . notify_all () ; } # [doc = " Returns the current number of items in the queue."] pub (crate) fn len (& self) -> usize { self . inner . queue . lock () . unwrap () . len () } # [doc = " Returns true if the queue is empty."] pub (crate) fn is_empty (& self) -> bool { self . inner . queue . lock () . unwrap () . is_empty () } }
    };
}

impl_123!()