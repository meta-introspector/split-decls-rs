macro_rules! deps {
    () => {
        Table!();
        VersionFile!();
    };
}

macro_rules! VersionFiles {
    () => {
        deps!();
        # [doc = " A table of filenames used for GNU versioning."] pub type VersionFiles < 'data > = Table < VersionFile < 'data > > ;
    };
}

VersionFiles!()