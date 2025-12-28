macro_rules! ExpectedCalls {
    () => {
        # [derive (PartialEq)] # [doc (hidden)] pub enum ExpectedCalls { Satisfied , TooMany , TooFew , }
    };
}

ExpectedCalls!();