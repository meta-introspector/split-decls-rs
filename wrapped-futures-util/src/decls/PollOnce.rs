macro_rules! PollOnce {
    () => {
        # [allow (missing_debug_implementations)] # [doc (hidden)] pub struct PollOnce < F : Future + Unpin > { future : F , }
    };
}

PollOnce!();