macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! ObjectFormat {
    () => {
        deps!();
        # [doc = " The `core.checkStat` key."] pub type ObjectFormat = keys :: Any < validate :: ObjectFormat > ;
    };
}

ObjectFormat!()