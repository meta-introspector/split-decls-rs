macro_rules! TargetFeatureIsSafeInTarget {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum TargetFeatureIsSafeInTarget { No , Yes , }
    };
}

TargetFeatureIsSafeInTarget!()