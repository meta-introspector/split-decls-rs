macro_rules! deps {
    () => {
        IAsyncActionWithProgress!();
        ReadyState!();
    };
}

macro_rules! ReadyActionWithProgress {
    () => {
        deps!();
        # [implement (IAsyncActionWithProgress < P >, IAsyncInfo)] struct ReadyActionWithProgress < P > (ReadyState < IAsyncActionWithProgress < P > >) where P : RuntimeType + 'static ;
    };
}

ReadyActionWithProgress!()