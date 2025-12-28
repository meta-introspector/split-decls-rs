macro_rules! home {
    () => {
        # [doc = " Path to the current test's `$HOME`"] # [doc = ""] # [doc = " ex: `$CARGO_TARGET_TMPDIR/cit/t0/home`"] pub fn home () -> PathBuf { let mut path = root () ; path . push ("home") ; path . mkdir_p () ; path }
    };
}

home!();