macro_rules! dl_path {
    () => {
        # [doc = " Path to download `.crate` files using the web API endpoint."] # [doc = ""] # [doc = " Crates"] # [doc = " should be organized as `{name}/{version}/download` to match the web API"] # [doc = " endpoint. This is rarely used and must be manually set up."] # [doc = ""] # [doc = " ex: `$CARGO_TARGET_TMPDIR/cit/t0/dl`"] pub fn dl_path () -> PathBuf { generate_path ("dl") }
    };
}

dl_path!();