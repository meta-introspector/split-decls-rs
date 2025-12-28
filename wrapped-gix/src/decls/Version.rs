macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Version {
    () => {
        deps!();
        # [doc = " The `protocol.version` key."] pub type Version = keys :: Any < validate :: Version > ;
    };
}

Version!()