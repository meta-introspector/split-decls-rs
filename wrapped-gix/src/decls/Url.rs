macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Url {
    () => {
        deps!();
        # [doc = " A key that represents a URL."] pub type Url = Any < validate :: Url > ;
    };
}

Url!()