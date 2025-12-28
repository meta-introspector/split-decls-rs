macro_rules! AsyncStdExecutor {
    () => {
        # [doc = " Runs futures on the 'async-std' crate's global executor"] # [cfg (feature = "async_std")] pub struct AsyncStdExecutor ;
    };
}

AsyncStdExecutor!();