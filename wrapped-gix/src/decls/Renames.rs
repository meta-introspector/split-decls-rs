macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Renames {
    () => {
        deps!();
        # [doc = " The `diff.renames` key."] pub type Renames = keys :: Any < validate :: Renames > ;
    };
}

Renames!();