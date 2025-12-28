macro_rules! deps {
    () => {
        ComPtr!();
    };
}

macro_rules! SetupConfiguration {
    () => {
        deps!();
        pub struct SetupConfiguration (ComPtr < ISetupConfiguration >) ;
    };
}

SetupConfiguration!()