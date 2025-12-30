// Generated macro for SubcommandCandidates (struct)
macro_rules! Depcrate_engine_customSubcommandCandidates {
() => {
// Module: crate::engine::custom
// Provides: {"SubcommandCandidates"}
// Dependencies: {}
# [doc = " Extend [`Command`][clap::Command] with a [`ValueCandidates`]"] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use clap::Parser;"] # [doc = " use clap_complete::engine::{SubcommandCandidates, CompletionCandidate};"] # [doc = " #[derive(Debug, Parser)]"] # [doc = " #[clap(name = \"cli\", add = SubcommandCandidates::new(|| { vec!["] # [doc = "     CompletionCandidate::new(\"foo\"),"] # [doc = "     CompletionCandidate::new(\"bar\"),"] # [doc = "     CompletionCandidate::new(\"baz\")] }))]"] # [doc = " struct Cli {"] # [doc = "     #[arg(long)]"] # [doc = "     input: Option<String>,"] # [doc = " }"] # [doc = " ```"] # [derive (Clone)] pub struct SubcommandCandidates (Arc < dyn ValueCandidates >) ;
};
}
