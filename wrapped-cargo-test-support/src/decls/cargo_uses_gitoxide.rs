macro_rules! cargo_uses_gitoxide {
    () => {
        # [doc = " Returns true if gitoxide is globally activated."] # [doc = ""] # [doc = " That way, tests that normally use `git2` can transparently use `gitoxide`."] pub fn cargo_uses_gitoxide () -> bool { std :: env :: var_os ("__CARGO_USE_GITOXIDE_INSTEAD_OF_GIT2") . map_or (false , | value | value == "1") }
    };
}

cargo_uses_gitoxide!();