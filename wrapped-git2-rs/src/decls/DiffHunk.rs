macro_rules! DiffHunk {
    () => {
        # [doc = " Structure describing a hunk of a diff."] pub struct DiffHunk < 'a > { raw : * const raw :: git_diff_hunk , _marker : marker :: PhantomData < & 'a raw :: git_diff_hunk > , }
    };
}

DiffHunk!()