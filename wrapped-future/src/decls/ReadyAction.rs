macro_rules! deps {
    () => {
        ReadyState!();
    };
}

macro_rules! ReadyAction {
    () => {
        deps!();
        # [implement (IAsyncAction , IAsyncInfo)] struct ReadyAction (ReadyState < IAsyncAction >) ;
    };
}

ReadyAction!()