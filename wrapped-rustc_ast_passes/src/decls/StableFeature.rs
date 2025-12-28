macro_rules! StableFeature {
    () => {
        pub (crate) struct StableFeature { pub name : Symbol , pub since : Symbol , }
    };
}

StableFeature!();