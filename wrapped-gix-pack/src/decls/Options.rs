macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! Options {
    () => {
        deps!();
        # [doc = " Options for use in [`multi_index::File::write_from_index_paths()`]."] pub struct Options { # [doc = " The kind of hash to use for objects and to expect in the input files."] pub object_hash : gix_hash :: Kind , }
    };
}

Options!()