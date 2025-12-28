macro_rules! tokens_for_diffing {
    () => {
        # [doc = " Return an iterator over tokens for use in diffing. These are usually lines, but it's important"] # [doc = " to unify them so the later access shows the right thing."] pub (crate) fn tokens_for_diffing (data : & [u8]) -> impl TokenSource < Token = & [u8] > { data . tokenize () }
    };
}

tokens_for_diffing!();