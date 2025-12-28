macro_rules! SmolExecutor {
    () => {
        # [doc = " Runs futures on the 'smol' crate's global executor"] # [cfg (feature = "async_smol")] pub struct SmolExecutor ;
    };
}

SmolExecutor!();