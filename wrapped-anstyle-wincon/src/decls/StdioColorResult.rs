macro_rules! StdioColorResult {
    () => {
        type StdioColorResult = std :: io :: Result < (anstyle :: AnsiColor , anstyle :: AnsiColor) > ;
    };
}

StdioColorResult!()