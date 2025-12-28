macro_rules! ProgressState {
    () => {
        pub (crate) enum ProgressState { Borrowed (* const raw :: git_indexer_progress) , Owned (raw :: git_indexer_progress) , }
    };
}

ProgressState!()