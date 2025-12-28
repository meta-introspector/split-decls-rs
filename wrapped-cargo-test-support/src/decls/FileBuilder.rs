macro_rules! FileBuilder {
    () => {
        # [derive (PartialEq , Clone)] struct FileBuilder { path : PathBuf , body : String , executable : bool , }
    };
}

FileBuilder!();