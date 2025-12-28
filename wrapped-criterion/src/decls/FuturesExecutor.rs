macro_rules! FuturesExecutor {
    () => {
        # [doc = " Runs futures on the 'futures' crate's built-in current-thread executor"] # [cfg (feature = "async_futures")] pub struct FuturesExecutor ;
    };
}

FuturesExecutor!();