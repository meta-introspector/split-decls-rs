macro_rules! home_dir {
    () => {
        # [doc = " Tries to obtain the home directory from `HOME` on all platforms, but falls back to"] # [doc = " [`std::env::home_dir()`] for more complex ways of obtaining a home directory, particularly useful"] # [doc = " on Windows."] # [doc = ""] # [doc = " The reason `HOME` is tried first is to allow Windows users to have a custom location for their"] # [doc = " linux-style home, as otherwise they would have to accumulate dot files in a directory these are"] # [doc = " inconvenient and perceived as clutter."] # [cfg (not (target_family = "wasm"))] pub fn home_dir () -> Option < PathBuf > { std :: env :: var_os ("HOME") . map (Into :: into) . or_else (std :: env :: home_dir) }
    };
}

home_dir!()