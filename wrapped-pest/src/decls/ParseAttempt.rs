macro_rules! deps {
    () => {
        ParserState!();
        ParseAttempts!();
        Token!();
    };
}

macro_rules! ParseAttempt {
    () => {
        deps!();
        # [doc = " Structure tracking errored parsing call (associated with specific `ParserState` function)."] # [derive (Debug , Hash , PartialEq , Eq , Clone , PartialOrd , Ord)] pub enum ParseAttempt < R > { # [doc = " Call of `rule` errored."] Rule (R) , # [doc = " Call of token element (e.g., `match_string` or `match_insensitive`) errored."] # [doc = " Works as indicator of that leaf node is not a rule. In order to get the token value we"] # [doc = " can address `ParseAttempts` `(un)expected_tokens`."] Token , }
    };
}

ParseAttempt!();