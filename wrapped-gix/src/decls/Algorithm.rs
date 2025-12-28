macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Algorithm {
    () => {
        deps!();
        # [doc = " The `diff.algorithm` key."] pub type Algorithm = keys :: Any < validate :: Algorithm > ;
    };
}

Algorithm!()