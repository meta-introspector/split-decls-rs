macro_rules! alt_api_path {
    () => {
        # [doc = " Path to the alternative-registry version of [`api_path`]"] # [doc = ""] # [doc = " ex: `$CARGO_TARGET_TMPDIR/cit/t0/alternative-api`"] pub fn alt_api_path () -> PathBuf { generate_path ("alternative-api") }
    };
}

alt_api_path!()