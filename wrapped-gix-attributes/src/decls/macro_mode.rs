macro_rules! macro_mode {
    () => {
        fn macro_mode () -> gix_glob :: pattern :: Mode { gix_glob :: pattern :: Mode :: all () }
    };
}

macro_mode!()