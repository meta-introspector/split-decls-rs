macro_rules! deps {
    () => {
        SyncState!();
        IAsyncOperationWithProgress!();
    };
}

macro_rules! OperationWithProgress {
    () => {
        deps!();
        # [implement (IAsyncOperationWithProgress < T , P >, IAsyncInfo)] struct OperationWithProgress < T , P > (SyncState < IAsyncOperationWithProgress < T , P > >) where T : RuntimeType + 'static , P : RuntimeType + 'static ;
    };
}

OperationWithProgress!()