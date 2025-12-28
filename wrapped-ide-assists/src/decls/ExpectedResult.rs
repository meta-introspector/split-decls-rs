macro_rules! ExpectedResult {
    () => {
        enum ExpectedResult < 'a > { NotApplicable , Unresolved , After (& 'a str) , Target (& 'a str) , }
    };
}

ExpectedResult!()