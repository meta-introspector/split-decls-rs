macro_rules! deps {
    () => {
        ComPtr!();
    };
}

macro_rules! SetupInstance {
    () => {
        deps!();
        pub struct SetupInstance (ComPtr < ISetupInstance >) ;
    };
}

SetupInstance!()