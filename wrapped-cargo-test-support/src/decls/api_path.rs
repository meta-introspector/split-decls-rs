macro_rules! api_path {
    () => {
        # [doc = " Path to the local web API uploads"] # [doc = ""] # [doc = " Cargo will place the contents of a web API"] # [doc = " request here. For example, `api/v1/crates/new` is the result of publishing a crate."] # [doc = ""] # [doc = " ex: `$CARGO_TARGET_TMPDIR/cit/t0/api`"] pub fn api_path () -> PathBuf { generate_path ("api") }
    };
}

api_path!()