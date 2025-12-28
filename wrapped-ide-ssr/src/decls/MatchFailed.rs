macro_rules! MatchFailed {
    () => {
        # [doc = " An \"error\" indicating that matching failed. Use the fail_match! macro to create and return this."] # [derive (Clone)] pub (crate) struct MatchFailed { # [doc = " The reason why we failed to match. Only present when debug_active true in call to"] # [doc = " `get_match`."] pub (crate) reason : Option < String > , }
    };
}

MatchFailed!();