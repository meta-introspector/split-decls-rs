macro_rules! deps {
    () => {
        DummyRegexCaptures!();
    };
}

macro_rules! CurrentRegexCaptures {
    () => {
        deps!();
        # [cfg (not (feature = "regex_enabled"))] pub type CurrentRegexCaptures < 't > = DummyRegexCaptures ;
    };
}

CurrentRegexCaptures!();