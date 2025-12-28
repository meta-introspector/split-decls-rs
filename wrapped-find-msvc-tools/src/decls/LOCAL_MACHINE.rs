macro_rules! deps {
    () => {
        RegistryKey!();
        Repr!();
    };
}

macro_rules! LOCAL_MACHINE {
    () => {
        deps!();
        pub (crate) const LOCAL_MACHINE : RegistryKey = RegistryKey (Repr :: LocalMachine) ;
    };
}

LOCAL_MACHINE!();