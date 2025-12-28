macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Boolean {
    () => {
        deps!();
        # [doc = " A key that represents a boolean value."] pub type Boolean = Any < validate :: Boolean > ;
    };
}

Boolean!()