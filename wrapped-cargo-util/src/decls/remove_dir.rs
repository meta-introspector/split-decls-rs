macro_rules! remove_dir {
    () => {
        # [doc = " Equivalent to [`std::fs::remove_dir`] with better error messages."] pub fn remove_dir < P : AsRef < Path > > (p : P) -> Result < () > { _remove_dir (p . as_ref ()) }
    };
}

remove_dir!();