macro_rules! raw_cargo {
    () => {
        fn raw_cargo () -> Command { Command :: new (option_env ! ("CARGO") . unwrap_or ("cargo")) }
    };
}

raw_cargo!()