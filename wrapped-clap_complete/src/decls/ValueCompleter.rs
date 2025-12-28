macro_rules! deps {
    () => {
        ArgValueCompleter!();
        CompletionCandidate!();
    };
}

macro_rules! ValueCompleter {
    () => {
        deps!();
        # [doc = " User-provided completion candidates for an [`Arg`][clap::Arg], see [`ArgValueCompleter`]"] # [doc = ""] # [doc = " This is useful when predefined value hints are not enough."] pub trait ValueCompleter : Send + Sync { # [doc = " All potential candidates for an argument."] # [doc = ""] # [doc = " See [`CompletionCandidate`] for more information."] fn complete (& self , current : & OsStr) -> Vec < CompletionCandidate > ; }
    };
}

ValueCompleter!()