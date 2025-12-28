macro_rules! create_dir_all {
    () => {
        # [doc = " Equivalent to [`std::fs::create_dir_all`] with better error messages."] pub fn create_dir_all (p : impl AsRef < Path >) -> Result < () > { _create_dir_all (p . as_ref ()) }
    };
}

create_dir_all!()