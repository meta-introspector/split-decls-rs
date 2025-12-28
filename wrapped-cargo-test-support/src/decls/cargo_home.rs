macro_rules! cargo_home {
    () => {
        # [doc = " Path to the current test's `$CARGO_HOME`"] # [doc = ""] # [doc = " ex: `$CARGO_TARGET_TMPDIR/cit/t0/home/.cargo`"] pub fn cargo_home () -> PathBuf { home () . join (".cargo") }
    };
}

cargo_home!();