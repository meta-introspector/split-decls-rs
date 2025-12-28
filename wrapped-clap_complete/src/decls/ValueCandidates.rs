macro_rules! deps {
    () => {
        CompletionCandidate!();
        SubcommandCandidates!();
        ArgValueCandidates!();
    };
}

macro_rules! ValueCandidates {
    () => {
        deps!();
        # [doc = " User-provided completion candidates for an [`Arg`][clap::Arg], see [`ArgValueCandidates`]"] # [doc = ""] # [doc = " User-provided completion candidates for an [`Subcommand`][clap::Subcommand], see [`SubcommandCandidates`]"] # [doc = ""] # [doc = " This is useful when predefined value hints are not enough."] pub trait ValueCandidates : Send + Sync { # [doc = " All potential candidates for an argument."] # [doc = ""] # [doc = " See [`CompletionCandidate`] for more information."] fn candidates (& self) -> Vec < CompletionCandidate > ; }
    };
}

ValueCandidates!()