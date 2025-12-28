macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! LockTimeout {
    () => {
        deps!();
        # [doc = " The `core.(filesRefLockTimeout|packedRefsTimeout)` keys, or any other lock timeout for that matter."] pub type LockTimeout = Any < validate :: LockTimeout > ;
    };
}

LockTimeout!()