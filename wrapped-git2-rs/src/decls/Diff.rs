macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! Diff {
    () => {
        deps!();
        # [doc = " The diff object that contains all individual file deltas."] # [doc = ""] # [doc = " This is an opaque structure which will be allocated by one of the diff"] # [doc = " generator functions on the `Repository` structure (e.g. `diff_tree_to_tree`"] # [doc = " or other `diff_*` functions)."] pub struct Diff < 'repo > { raw : * mut raw :: git_diff , _marker : marker :: PhantomData < & 'repo Repository > , }
    };
}

Diff!()