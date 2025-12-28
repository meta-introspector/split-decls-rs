macro_rules! Indexer {
    () => {
        # [doc = " Generate an iteration sequence. This provides *fair* iteration when multiple"] # [doc = " futures need to be polled concurrently."] pub (crate) struct Indexer { offset : usize , max : usize , }
    };
}

Indexer!()