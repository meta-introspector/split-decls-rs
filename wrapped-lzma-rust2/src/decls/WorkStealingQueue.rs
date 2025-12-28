macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! WorkStealingQueue {
    () => {
        deps!();
        # [doc = " A work-stealing queue that supports multiple workers taking work from a shared queue."] # [doc = ""] # [doc = " Will be removed once core::sync::mpsc is stable."] pub (crate) struct WorkStealingQueue < T > { inner : Arc < Inner < T > > , }
    };
}

WorkStealingQueue!()