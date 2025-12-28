macro_rules! ICE_PATH {
    () => {
        static ICE_PATH : OnceLock < Option < PathBuf > > = OnceLock :: new () ;
    };
}

ICE_PATH!()