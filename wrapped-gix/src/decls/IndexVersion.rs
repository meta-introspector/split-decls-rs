macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! IndexVersion {
    () => {
        deps!();
        # [doc = " The `pack.indexVersion` key."] pub type IndexVersion = keys :: Any < validate :: IndexVersion > ;
    };
}

IndexVersion!()