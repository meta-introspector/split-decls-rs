macro_rules! deps {
    () => {
        Outcome!();
    };
}

macro_rules! WriteOutcome {
    () => {
        deps!();
        struct WriteOutcome { outcome : crate :: index :: write :: Outcome , data_path : Option < PathBuf > , index_path : Option < PathBuf > , keep_path : Option < PathBuf > , }
    };
}

WriteOutcome!();