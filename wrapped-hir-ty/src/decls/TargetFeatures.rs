macro_rules! TargetFeatures {
    () => {
        # [derive (Debug , Default , Clone)] pub struct TargetFeatures { pub (crate) enabled : FxHashSet < Symbol > , }
    };
}

TargetFeatures!();