macro_rules! deps {
    () => {
        File!();
        Error!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        # [doc = " Initialization"] impl File { # [doc = " Open the multi-index file at the given `path`."] pub fn at (path : impl AsRef < Path >) -> Result < Self , Error > { Self :: try_from (path . as_ref ()) } }
    };
}

impl_311!();