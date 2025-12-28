macro_rules! deps {
    () => {
        ReleaseChannel!();
        Crate!();
        RootQueryDb!();
    };
}

macro_rules! toolchain_channel {
    () => {
        deps!();
        fn toolchain_channel (db : & dyn RootQueryDb , krate : Crate) -> Option < ReleaseChannel > { krate . workspace_data (db) . toolchain . as_ref () . and_then (| v | ReleaseChannel :: from_str (& v . pre)) }
    };
}

toolchain_channel!();