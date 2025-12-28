macro_rules! deps {
    () => {
        Build!();
        Config!();
    };
}

macro_rules! make_config {
    () => {
        deps!();
        fn make_config () -> Config { Config { build : Build { rustflags : rustflags :: make_vec () , } , } }
    };
}

make_config!();