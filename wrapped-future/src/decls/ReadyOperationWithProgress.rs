macro_rules! deps {
    () => {
        ReadyState!();
        IAsyncOperationWithProgress!();
    };
}

macro_rules! ReadyOperationWithProgress {
    () => {
        deps!();
        # [implement (IAsyncOperationWithProgress < T , P >, IAsyncInfo)] struct ReadyOperationWithProgress < T , P > (ReadyState < IAsyncOperationWithProgress < T , P > >) where T : RuntimeType + 'static , P : RuntimeType + 'static ;
    };
}

ReadyOperationWithProgress!()