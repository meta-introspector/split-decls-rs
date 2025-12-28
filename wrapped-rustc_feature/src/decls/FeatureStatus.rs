macro_rules! FeatureStatus {
    () => {
        # [derive (PartialEq)] enum FeatureStatus { Default , Incomplete , Internal , }
    };
}

FeatureStatus!()