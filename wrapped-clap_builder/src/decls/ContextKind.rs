macro_rules! deps {
    () => {
        Usage!();
    };
}

macro_rules! ContextKind {
    () => {
        deps!();
        # [doc = " Semantics for a piece of error information"] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] # [non_exhaustive] # [cfg (feature = "error-context")] pub enum ContextKind { # [doc = " The cause of the error"] InvalidSubcommand , # [doc = " The cause of the error"] InvalidArg , # [doc = " Existing arguments"] PriorArg , # [doc = " Accepted subcommands"] ValidSubcommand , # [doc = " Accepted values"] ValidValue , # [doc = " Rejected values"] InvalidValue , # [doc = " Number of values present"] ActualNumValues , # [doc = " Number of allowed values"] ExpectedNumValues , # [doc = " Minimum number of allowed values"] MinValues , # [doc = " Potential fix for the user"] SuggestedCommand , # [doc = " Potential fix for the user"] SuggestedSubcommand , # [doc = " Potential fix for the user"] SuggestedArg , # [doc = " Potential fix for the user"] SuggestedValue , # [doc = " Trailing argument"] TrailingArg , # [doc = " Potential fix for the user"] Suggested , # [doc = " A usage string"] Usage , # [doc = " An opaque message to the user"] Custom , }
    };
}

ContextKind!();