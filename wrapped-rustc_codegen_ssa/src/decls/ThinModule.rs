macro_rules! deps {
    () => {
        ThinShared!();
        WriteBackendMethods!();
    };
}

macro_rules! ThinModule {
    () => {
        deps!();
        pub struct ThinModule < B : WriteBackendMethods > { pub shared : Arc < ThinShared < B > > , pub idx : usize , }
    };
}

ThinModule!();