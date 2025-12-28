macro_rules! deps {
    () => {
        StdError!();
    };
}

macro_rules! BoxedError {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] # [repr (transparent)] pub struct BoxedError (pub Box < dyn StdError + Send + Sync >) ;
    };
}

BoxedError!();