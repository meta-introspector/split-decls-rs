macro_rules! PathAncestors {
    () => {
        pub struct PathAncestors < 'a > { current : Option < & 'a Path > , stop_at : Option < PathBuf > , }
    };
}

PathAncestors!();