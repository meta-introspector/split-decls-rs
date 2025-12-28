macro_rules! deps {
    () => {
        WorkerThread!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl Drop for WorkerThread { fn drop (& mut self) { WORKER_THREAD_STATE . with (| t | { assert ! (t . get () . eq (& (self as * const _))) ; t . set (ptr :: null ()) ; }) ; } }
    };
}

impl_124!()