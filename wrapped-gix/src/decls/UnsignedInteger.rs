macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! UnsignedInteger {
    () => {
        deps!();
        # [doc = " A key which represents any unsigned integer."] pub type UnsignedInteger = Any < validate :: UnsignedInteger > ;
    };
}

UnsignedInteger!()