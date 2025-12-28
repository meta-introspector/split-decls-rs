macro_rules! ParseError {
    () => {
        # [derive (PartialEq , Eq , Debug)] pub enum ParseError { # [doc = " Symbol doesn't match the expected `v0` grammar."] Invalid , # [doc = " Parsing the symbol crossed the recursion limit (see `MAX_DEPTH`)."] RecursedTooDeep , }
    };
}

ParseError!();