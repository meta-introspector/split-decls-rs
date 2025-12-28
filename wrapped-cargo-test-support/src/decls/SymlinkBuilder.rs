macro_rules! SymlinkBuilder {
    () => {
        # [derive (PartialEq , Clone)] struct SymlinkBuilder { dst : PathBuf , src : PathBuf , src_is_dir : bool , }
    };
}

SymlinkBuilder!();