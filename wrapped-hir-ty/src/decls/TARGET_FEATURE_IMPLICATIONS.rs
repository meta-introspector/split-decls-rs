macro_rules! TARGET_FEATURE_IMPLICATIONS {
    () => {
        static TARGET_FEATURE_IMPLICATIONS : LazyLock < FxHashMap < Symbol , Box < [Symbol] > > > = LazyLock :: new (| | { let mut result = FxHashMap :: < Symbol , FxHashSet < Symbol > > :: default () ; for & (feature_str , implications) in TARGET_FEATURE_IMPLICATIONS_RAW { let feature = Symbol :: intern (feature_str) ; let implications = implications . iter () . copied () . map (Symbol :: intern) ; result . entry (feature) . or_default () . extend (implications) ; } let mut result = result . into_iter () . map (| (feature , implications) | (feature , Box :: from_iter (implications))) . collect :: < FxHashMap < _ , _ > > () ; result . shrink_to_fit () ; result }) ;
    };
}

TARGET_FEATURE_IMPLICATIONS!()