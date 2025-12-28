macro_rules! GLOBAL_ROOT {
    () => {
        static GLOBAL_ROOT : OnceLock < Mutex < Option < PathBuf > > > = OnceLock :: new () ;
    };
}

GLOBAL_ROOT!()