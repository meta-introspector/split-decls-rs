macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! PushRefSpec {
    () => {
        deps!();
        # [doc = " A key that represents a `RefSpec` for pushing."] pub type PushRefSpec = Any < validate :: PushRefSpec > ;
    };
}

PushRefSpec!();