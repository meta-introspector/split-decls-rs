macro_rules! deps {
    () => {
        SyncState!();
    };
}

macro_rules! Action {
    () => {
        deps!();
        # [implement (IAsyncAction , IAsyncInfo)] struct Action (SyncState < IAsyncAction >) ;
    };
}

Action!()