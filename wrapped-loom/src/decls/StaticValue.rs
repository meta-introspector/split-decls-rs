macro_rules! deps {
    () => {
        Synchronize!();
    };
}

macro_rules! StaticValue {
    () => {
        deps!();
        pub (crate) struct StaticValue { pub (crate) sync : Synchronize , v : Box < dyn Any > , }
    };
}

StaticValue!();