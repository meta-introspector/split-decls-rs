macro_rules! deps {
    () => {
        ThreadSafeRepository!();
        Options!();
        Error!();
        Repository!();
    };
}

macro_rules! open_opts {
    () => {
        deps!();
        # [doc = " See [`ThreadSafeRepository::open_opts()`], but returns a [`Repository`] instead."] # [allow (clippy :: result_large_err)] # [doc (alias = "open_ext" , alias = "git2")] pub fn open_opts (directory : impl Into < std :: path :: PathBuf > , options : open :: Options) -> Result < Repository , open :: Error > { ThreadSafeRepository :: open_opts (directory , options) . map (Into :: into) }
    };
}

open_opts!()