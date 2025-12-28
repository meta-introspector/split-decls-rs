macro_rules! deps {
    () => {
        NFA!();
        BuildErrorKind!();
    };
}

macro_rules! BuildError {
    () => {
        deps!();
        # [doc = " An error that can occurred during the construction of a thompson NFA."] # [doc = ""] # [doc = " This error does not provide many introspection capabilities. There are"] # [doc = " generally only two things you can do with it:"] # [doc = ""] # [doc = " * Obtain a human readable message via its `std::fmt::Display` impl."] # [doc = " * Access an underlying [`regex_syntax::Error`] type from its `source`"] # [doc = " method via the `std::error::Error` trait. This error only occurs when using"] # [doc = " convenience routines for building an NFA directly from a pattern string."] # [doc = ""] # [doc = " Otherwise, errors typically occur when a limit has been breached. For"] # [doc = " example, if the total heap usage of the compiled NFA exceeds the limit"] # [doc = " set by [`Config::nfa_size_limit`](crate::nfa::thompson::Config), then"] # [doc = " building the NFA will fail."] # [derive (Clone , Debug)] pub struct BuildError { kind : BuildErrorKind , }
    };
}

BuildError!();