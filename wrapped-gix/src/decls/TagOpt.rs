macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! TagOpt {
    () => {
        deps!();
        # [doc = " The `remote.<name>.tagOpt` key type."] pub type TagOpt = keys :: Any < validate :: TagOpt > ;
    };
}

TagOpt!()