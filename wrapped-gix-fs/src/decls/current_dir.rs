macro_rules! current_dir {
    () => {
        # [doc = " Like [`std::env::current_dir()`], but it will `precompose_unicode` if that value is true, if the current directory"] # [doc = " is valid unicode and if there are decomposed unicode codepoints."] # [doc = ""] # [doc = " Thus, it will turn `\"a\\u{308}\"` into `ä` if `true`."] # [doc = " Keeping it `false` will not alter the output."] # [doc = ""] # [doc = " Note that `precompose_unicode` most be set using the `core.precomposeUnicode` git configuration."] pub fn current_dir (precompose_unicode : bool) -> std :: io :: Result < PathBuf > { let cwd = std :: env :: current_dir () ? ; Ok (if precompose_unicode { gix_utils :: str :: precompose_path (cwd . into ()) . into_owned () } else { cwd }) }
    };
}

current_dir!();