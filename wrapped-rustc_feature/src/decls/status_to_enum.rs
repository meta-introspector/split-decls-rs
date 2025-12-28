macro_rules! deps {
    () => {
        FeatureStatus!();
    };
}

macro_rules! status_to_enum {
    () => {
        deps!();
        macro_rules ! status_to_enum { (unstable) => { FeatureStatus :: Default } ; (incomplete) => { FeatureStatus :: Incomplete } ; (internal) => { FeatureStatus :: Internal } ; }
    };
}

status_to_enum!()