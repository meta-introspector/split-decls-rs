macro_rules! deps {
    () => {
        ComPtr!();
    };
}

macro_rules! EnumSetupInstances {
    () => {
        deps!();
        pub struct EnumSetupInstances (ComPtr < IEnumSetupInstances >) ;
    };
}

EnumSetupInstances!();