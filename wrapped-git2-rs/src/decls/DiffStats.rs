macro_rules! DiffStats {
    () => {
        # [doc = " Structure describing a hunk of a diff."] pub struct DiffStats { raw : * mut raw :: git_diff_stats , }
    };
}

DiffStats!();