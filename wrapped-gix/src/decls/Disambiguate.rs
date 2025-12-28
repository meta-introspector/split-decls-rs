macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Disambiguate {
    () => {
        deps!();
        # [doc = " The `core.disambiguate` key."] pub type Disambiguate = keys :: Any < validate :: Disambiguate > ;
    };
}

Disambiguate!();