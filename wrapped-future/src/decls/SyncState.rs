macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! SyncState {
    () => {
        deps!();
        struct SyncState < T : Async > (Mutex < State < T > >) ;
    };
}

SyncState!();