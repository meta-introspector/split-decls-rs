macro_rules! deps {
    () => {
        WorkerThread!();
    };
}

macro_rules! macro_122 {
    () => {
        deps!();
        thread_local ! { static WORKER_THREAD_STATE : Cell <* const WorkerThread > = const { Cell :: new (ptr :: null ()) } ; }
    };
}

macro_122!()