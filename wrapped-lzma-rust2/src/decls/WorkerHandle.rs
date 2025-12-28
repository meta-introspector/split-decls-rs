macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! WorkerHandle {
    () => {
        deps!();
        # [doc = " A handle for workers to steal work from the queue."] pub (crate) struct WorkerHandle < T > { inner : Arc < Inner < T > > , }
    };
}

WorkerHandle!()