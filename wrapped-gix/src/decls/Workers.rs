macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Workers {
    () => {
        deps!();
        # [doc = " The `checkout.workers` key."] pub type Workers = keys :: Any < validate :: Workers > ;
    };
}

Workers!();