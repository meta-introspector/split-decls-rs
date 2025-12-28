macro_rules! PathWrapper {
    () => {
        # [derive (Debug)] struct PathWrapper { path : PathBuf , is_directory : bool , }
    };
}

PathWrapper!();