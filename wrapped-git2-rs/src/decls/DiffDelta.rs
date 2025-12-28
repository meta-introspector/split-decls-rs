macro_rules! DiffDelta {
    () => {
        # [doc = " Description of changes to one entry."] pub struct DiffDelta < 'a > { raw : * mut raw :: git_diff_delta , _marker : marker :: PhantomData < & 'a raw :: git_diff_delta > , }
    };
}

DiffDelta!();