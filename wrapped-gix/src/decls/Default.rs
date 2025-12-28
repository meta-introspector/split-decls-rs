macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Default {
    () => {
        deps!();
        # [doc = " The `remote.<name>.tagOpt` key type."] pub type Default = keys :: Any < validate :: Default > ;
    };
}

Default!()