macro_rules! LexError {
    () => {
        # [doc = " Error returned from `TokenStream::from_str`."] pub struct LexError { inner : imp :: LexError , _marker : ProcMacroAutoTraits , }
    };
}

LexError!()