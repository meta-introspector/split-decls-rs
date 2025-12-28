macro_rules! deps {
    () => {
        ReadyState!();
        IAsyncOperation!();
    };
}

macro_rules! ReadyOperation {
    () => {
        deps!();
        # [implement (IAsyncOperation < T >, IAsyncInfo)] struct ReadyOperation < T > (ReadyState < IAsyncOperation < T > >) where T : RuntimeType + 'static ;
    };
}

ReadyOperation!()