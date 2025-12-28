macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Ignore {
    () => {
        deps!();
        # [doc = " The `diff.ignoreSubmodules` key."] pub type Ignore = keys :: Any < validate :: Ignore > ;
    };
}

Ignore!()