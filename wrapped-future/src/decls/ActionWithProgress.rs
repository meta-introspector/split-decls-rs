macro_rules! deps {
    () => {
        SyncState!();
        IAsyncActionWithProgress!();
    };
}

macro_rules! ActionWithProgress {
    () => {
        deps!();
        # [implement (IAsyncActionWithProgress < P >, IAsyncInfo)] struct ActionWithProgress < P > (SyncState < IAsyncActionWithProgress < P > >) where P : RuntimeType + 'static ;
    };
}

ActionWithProgress!();