macro_rules! FutureOrOutput {
    () => {
        enum FutureOrOutput < Fut : Future > { Future (Fut) , Output (Fut :: Output) , }
    };
}

FutureOrOutput!();