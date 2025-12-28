macro_rules! FeatureMap {
    () => {
        pub (crate) type FeatureMap = BTreeMap < String , Vec < String > > ;
    };
}

FeatureMap!();