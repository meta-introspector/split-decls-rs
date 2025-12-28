macro_rules! deps {
    () => {
        Patchwork!();
    };
}

macro_rules! FileRuntime {
    () => {
        deps!();
        struct FileRuntime { path : PathBuf , original_text : String , patchwork : Patchwork , }
    };
}

FileRuntime!()