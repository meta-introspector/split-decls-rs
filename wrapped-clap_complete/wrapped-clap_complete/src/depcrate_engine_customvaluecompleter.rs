// Generated macro for ValueCompleter (trait)
macro_rules! Depcrate_engine_customValueCompleter {
() => {
// Module: crate::engine::custom
// Provides: {"ValueCompleter"}
// Dependencies: {}
# [doc = " User-provided completion candidates for an [`Arg`][clap::Arg], see [`ArgValueCompleter`]"] # [doc = ""] # [doc = " This is useful when predefined value hints are not enough."] pub trait ValueCompleter : Send + Sync { # [doc = " All potential candidates for an argument."] # [doc = ""] # [doc = " See [`CompletionCandidate`] for more information."] fn complete (& self , current : & OsStr) -> Vec < CompletionCandidate > ; }
};
}
