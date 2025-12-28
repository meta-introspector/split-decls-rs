macro_rules! BlameHunk {
    () => {
        # [doc = " Structure that represents a blame hunk."] pub struct BlameHunk < 'blame > { raw : * mut raw :: git_blame_hunk , _marker : marker :: PhantomData < & 'blame raw :: git_blame > , }
    };
}

BlameHunk!()