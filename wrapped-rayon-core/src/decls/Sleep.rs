macro_rules! deps {
    () => {
        WorkerSleepState!();
        AtomicCounters!();
    };
}

macro_rules! Sleep {
    () => {
        deps!();
        # [doc = " The `Sleep` struct is embedded into each registry. It governs the waking and sleeping"] # [doc = " of workers. It has callbacks that are invoked periodically at significant events,"] # [doc = " such as when workers are looping and looking for work, when latches are set, or when"] # [doc = " jobs are published, and it either blocks threads or wakes them in response to these"] # [doc = " events. See the [`README.md`] in this module for more details."] # [doc = ""] # [doc = " [`README.md`] README.md"] pub (super) struct Sleep { # [doc = " One \"sleep state\" per worker. Used to track if a worker is sleeping and to have"] # [doc = " them block."] worker_sleep_states : Vec < CachePadded < WorkerSleepState > > , counters : AtomicCounters , }
    };
}

Sleep!()