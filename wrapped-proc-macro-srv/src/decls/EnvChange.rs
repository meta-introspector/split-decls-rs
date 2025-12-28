macro_rules! deps {
    () => {
        EnvSnapshot!();
    };
}

macro_rules! EnvChange {
    () => {
        deps!();
        struct EnvChange < 'snap > { changed_vars : Vec < & 'snap str > , prev_working_dir : Option < PathBuf > , snap : & 'snap EnvSnapshot , _guard : std :: sync :: MutexGuard < 'snap , () > , }
    };
}

EnvChange!()