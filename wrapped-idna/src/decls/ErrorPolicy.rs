macro_rules! ErrorPolicy {
    () => {
        # [doc = " Policy for customizing behavior in case of an error."] # [derive (PartialEq , Eq , Copy , Clone)] # [non_exhaustive] pub enum ErrorPolicy { # [doc = " Return as early as possible without producing output in case of error."] FailFast , # [doc = " In case of error, mark errors with the REPLACEMENT CHARACTER. (The output"] # [doc = " containing REPLACEMENT CHARACTERs may be show to the user to illustrate"] # [doc = " what was wrong but must not be used for naming in a network protocol.)"] MarkErrors , }
    };
}

ErrorPolicy!();