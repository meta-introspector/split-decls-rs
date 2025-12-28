macro_rules! deps {
    () => {
        OsEnv!();
    };
}

macro_rules! OS_ENV {
    () => {
        deps!();
        pub const OS_ENV : OsEnv = OsEnv { } ;
    };
}

OS_ENV!();