macro_rules! Config {
    () => {
        pub trait Config { const USE_FAST : bool ; }
    };
}

Config!();