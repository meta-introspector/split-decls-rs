macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! at {
    () => {
        deps!();
        # [doc = " Create a new cached handle to the object store."] pub fn at (objects_dir : impl Into < PathBuf >) -> std :: io :: Result < Handle > { at_opts (objects_dir , Vec :: new () , Default :: default ()) }
    };
}

at!()