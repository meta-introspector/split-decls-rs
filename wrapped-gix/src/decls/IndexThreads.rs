macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! IndexThreads {
    () => {
        deps!();
        # [doc = " The `index.threads` key."] pub type IndexThreads = keys :: Any < validate :: IndexThreads > ;
    };
}

IndexThreads!();