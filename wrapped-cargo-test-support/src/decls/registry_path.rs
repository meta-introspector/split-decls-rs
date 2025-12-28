macro_rules! registry_path {
    () => {
        # [doc = " Path to the local index for pseudo-crates.io."] # [doc = ""] # [doc = " This is a Git repo"] # [doc = " initialized with a `config.json` file pointing to `dl_path` for downloads"] # [doc = " and `api_path` for uploads."] # [doc = ""] # [doc = " ex: `$CARGO_TARGET_TMPDIR/cit/t0/registry`"] pub fn registry_path () -> PathBuf { generate_path ("registry") }
    };
}

registry_path!();