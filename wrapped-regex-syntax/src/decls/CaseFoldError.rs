macro_rules! CaseFoldError {
    () => {
        # [doc = " An error that occurs when Unicode-aware simple case folding fails."] # [doc = ""] # [doc = " This error can occur when the case mapping tables necessary for Unicode"] # [doc = " aware case folding are unavailable. This only occurs when the"] # [doc = " `unicode-case` feature is disabled. (The feature is enabled by default.)"] # [derive (Debug)] pub struct CaseFoldError (()) ;
    };
}

CaseFoldError!();