macro_rules! DiffLine {
    () => {
        # [doc = " Structure describing a line (or data span) of a diff."] pub struct DiffLine < 'a > { raw : * const raw :: git_diff_line , _marker : marker :: PhantomData < & 'a raw :: git_diff_line > , }
    };
}

DiffLine!();