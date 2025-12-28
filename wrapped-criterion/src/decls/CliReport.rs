macro_rules! deps {
    () => {
        CliVerbosity!();
    };
}

macro_rules! CliReport {
    () => {
        deps!();
        pub (crate) struct CliReport { pub enable_text_overwrite : bool , pub enable_text_coloring : bool , pub verbosity : CliVerbosity , }
    };
}

CliReport!();