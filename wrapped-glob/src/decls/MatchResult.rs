macro_rules! MatchResult {
    () => {
        # [derive (Copy , Clone , PartialEq)] enum MatchResult { Match , SubPatternDoesntMatch , EntirePatternDoesntMatch , }
    };
}

MatchResult!()