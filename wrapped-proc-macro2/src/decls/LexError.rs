macro_rules! deps {
    () => {
        ProcMacroAutoTraits!();
    };
}

macro_rules! LexError {
    () => {
        deps!();
        # [doc = " Error returned from `TokenStream::from_str`."] pub struct LexError { inner : imp :: LexError , _marker : ProcMacroAutoTraits , }
    };
}

LexError!();