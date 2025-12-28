macro_rules! alt_dl_path {
    () => {
        # [doc = " Path to the alternative-registry version of [`dl_path`]"] # [doc = ""] # [doc = " ex: `$CARGO_TARGET_TMPDIR/cit/t0/alternative-dl`"] pub fn alt_dl_path () -> PathBuf { generate_path ("alternative-dl") }
    };
}

alt_dl_path!();