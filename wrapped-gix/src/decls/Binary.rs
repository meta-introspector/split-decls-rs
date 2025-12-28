macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Binary {
    () => {
        deps!();
        # [doc = " The `diff.<driver>.binary` key."] pub type Binary = keys :: Any < validate :: Binary > ;
    };
}

Binary!();