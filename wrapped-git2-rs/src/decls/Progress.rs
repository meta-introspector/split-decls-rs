macro_rules! deps {
    () => {
        ProgressState!();
    };
}

macro_rules! Progress {
    () => {
        deps!();
        # [doc = " Struct representing the progress by an in-flight transfer."] pub struct Progress < 'a > { pub (crate) raw : ProgressState , pub (crate) _marker : marker :: PhantomData < & 'a raw :: git_indexer_progress > , }
    };
}

Progress!()