macro_rules! deps {
    () => {
        XorShift64Star!();
        JobFifo!();
        Registry!();
        JobRef!();
    };
}

macro_rules! WorkerThread {
    () => {
        deps!();
        pub (super) struct WorkerThread { # [doc = " the \"worker\" half of our local deque"] worker : Worker < JobRef > , # [doc = " the \"stealer\" half of the worker's broadcast deque"] stealer : Stealer < JobRef > , # [doc = " local queue used for `spawn_fifo` indirection"] fifo : JobFifo , index : usize , # [doc = " A weak random number generator."] rng : XorShift64Star , registry : Arc < Registry > , }
    };
}

WorkerThread!();