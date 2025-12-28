macro_rules! deps {
    () => {
        AnyValueId!();
        AnyValue!();
    };
}

macro_rules! MatchesError {
    () => {
        deps!();
        # [doc = " Violation of [`ArgMatches`][crate::ArgMatches] assumptions"] # [derive (Clone , Debug)] # [allow (missing_copy_implementations)] # [non_exhaustive] pub enum MatchesError { # [doc = " Failed to downcast `AnyValue` to the specified type"] # [non_exhaustive] Downcast { # [doc = " Type for value stored in [`ArgMatches`][crate::ArgMatches]"] actual : AnyValueId , # [doc = " The target type to downcast to"] expected : AnyValueId , } , # [doc = " Argument not defined in [`Command`][crate::Command]"] # [non_exhaustive] UnknownArgument { } , }
    };
}

MatchesError!();