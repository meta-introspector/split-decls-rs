macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! FetchRefSpec {
    () => {
        deps!();
        # [doc = " A key that represents a `RefSpec` for fetching."] pub type FetchRefSpec = Any < validate :: FetchRefSpec > ;
    };
}

FetchRefSpec!()