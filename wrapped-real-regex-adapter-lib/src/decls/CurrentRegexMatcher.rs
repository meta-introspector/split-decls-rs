macro_rules! deps {
    () => {
        DummyRegexMatcher!();
    };
}

macro_rules! CurrentRegexMatcher {
    () => {
        deps!();
        # [cfg (not (feature = "regex_enabled"))] pub type CurrentRegexMatcher = DummyRegexMatcher ;
    };
}

CurrentRegexMatcher!()