macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! CheckStat {
    () => {
        deps!();
        # [doc = " The `core.checkStat` key."] pub type CheckStat = keys :: Any < validate :: CheckStat > ;
    };
}

CheckStat!()