macro_rules! UnstableFeature {
    () => {
        pub (crate) struct UnstableFeature { pub msg : DiagMessage , }
    };
}

UnstableFeature!()