macro_rules! deps {
    () => {
        UnexpectedCfgRustcHelp!();
    };
}

macro_rules! impl_562 {
    () => {
        deps!();
        impl UnexpectedCfgRustcHelp { pub (crate) fn new (unescaped : & str) -> Self { Self { cmdline_arg : format ! ("--check-cfg={unescaped}") } } }
    };
}

impl_562!();