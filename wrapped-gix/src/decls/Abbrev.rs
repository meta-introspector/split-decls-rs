macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Abbrev {
    () => {
        deps!();
        # [doc = " The `core.abbrev` key."] pub type Abbrev = keys :: Any < validate :: Abbrev > ;
    };
}

Abbrev!()