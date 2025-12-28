macro_rules! CliVerbosity {
    () => {
        # [derive (Debug , Clone , Copy , Eq , PartialEq)] pub (crate) enum CliVerbosity { Quiet , Normal , Verbose , }
    };
}

CliVerbosity!()