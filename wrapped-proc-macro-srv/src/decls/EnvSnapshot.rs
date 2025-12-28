macro_rules! EnvSnapshot {
    () => {
        pub struct EnvSnapshot { vars : HashMap < OsString , OsString > , }
    };
}

EnvSnapshot!();