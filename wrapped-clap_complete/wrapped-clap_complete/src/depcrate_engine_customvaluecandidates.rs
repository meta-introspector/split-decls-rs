// Generated macro for ValueCandidates (trait)
macro_rules! Depcrate_engine_customValueCandidates {
() => {
// Module: crate::engine::custom
// Provides: {"ValueCandidates"}
// Dependencies: {}
# [doc = " User-provided completion candidates for an [`Arg`][clap::Arg], see [`ArgValueCandidates`]"] # [doc = ""] # [doc = " User-provided completion candidates for an [`Subcommand`][clap::Subcommand], see [`SubcommandCandidates`]"] # [doc = ""] # [doc = " This is useful when predefined value hints are not enough."] pub trait ValueCandidates : Send + Sync { # [doc = " All potential candidates for an argument."] # [doc = ""] # [doc = " See [`CompletionCandidate`] for more information."] fn candidates (& self) -> Vec < CompletionCandidate > ; }
};
}
