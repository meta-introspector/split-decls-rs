macro_rules! rustc_version_str {
    () => {
        # [doc = " Returns the version string for `rustc` itself (which may be different from a tool version)."] pub fn rustc_version_str () -> Option < & 'static str > { version_str ! () }
    };
}

rustc_version_str!()