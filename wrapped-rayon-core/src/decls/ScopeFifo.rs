macro_rules! deps {
    () => {
        ScopeBase!();
        JobFifo!();
    };
}

macro_rules! ScopeFifo {
    () => {
        deps!();
        # [doc = " Represents a fork-join scope which can be used to spawn any number of tasks."] # [doc = " Those spawned from the same thread are prioritized in relative FIFO order."] # [doc = " See [`scope_fifo()`] for more information."] pub struct ScopeFifo < 'scope > { base : ScopeBase < 'scope > , fifos : Vec < JobFifo > , }
    };
}

ScopeFifo!();