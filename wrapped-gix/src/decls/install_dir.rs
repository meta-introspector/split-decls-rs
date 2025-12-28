macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! install_dir {
    () => {
        deps!();
        pub (crate) fn install_dir () -> std :: io :: Result < PathBuf > { std :: env :: current_exe () . and_then (| exe | { exe . parent () . map (ToOwned :: to_owned) . ok_or_else (| | std :: io :: Error :: other ("no parent for current executable")) }) }
    };
}

install_dir!();