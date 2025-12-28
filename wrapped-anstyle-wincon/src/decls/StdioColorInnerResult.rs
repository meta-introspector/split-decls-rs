macro_rules! StdioColorInnerResult {
    () => {
        type StdioColorInnerResult = Result < (anstyle :: AnsiColor , anstyle :: AnsiColor) , inner :: IoError > ;
    };
}

StdioColorInnerResult!();