macro_rules! Pathspec {
    () => {
        # [doc = " Structure representing a compiled pathspec used for matching against various"] # [doc = " structures."] pub struct Pathspec { raw : * mut raw :: git_pathspec , }
    };
}

Pathspec!();