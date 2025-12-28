macro_rules! deps {
    () => {
        SyncState!();
        IAsyncOperation!();
    };
}

macro_rules! Operation {
    () => {
        deps!();
        # [implement (IAsyncOperation < T >, IAsyncInfo)] struct Operation < T > (SyncState < IAsyncOperation < T > >) where T : RuntimeType + 'static ;
    };
}

Operation!();