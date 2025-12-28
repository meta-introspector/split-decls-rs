macro_rules! DiffBinary {
    () => {
        # [doc = " Structure describing the binary contents of a diff."] pub struct DiffBinary < 'a > { raw : * const raw :: git_diff_binary , _marker : marker :: PhantomData < & 'a raw :: git_diff_binary > , }
    };
}

DiffBinary!();