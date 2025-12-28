macro_rules! deps {
    () => {
        ValueCandidates!();
        CompletionCandidate!();
    };
}

macro_rules! SubcommandCandidates {
    () => {
        deps!();
        # [doc = " Extend [`Command`][clap::Command] with a [`ValueCandidates`]"] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use clap::Parser;"] # [doc = " use clap_complete::engine::{SubcommandCandidates, CompletionCandidate};"] # [doc = " #[derive(Debug, Parser)]"] # [doc = " #[clap(name = \"cli\", add = SubcommandCandidates::new(|| { vec!["] # [doc = "     CompletionCandidate::new(\"foo\"),"] # [doc = "     CompletionCandidate::new(\"bar\"),"] # [doc = "     CompletionCandidate::new(\"baz\")] }))]"] # [doc = " struct Cli {"] # [doc = "     #[arg(long)]"] # [doc = "     input: Option<String>,"] # [doc = " }"] # [doc = " ```"] # [derive (Clone)] pub struct SubcommandCandidates (Arc < dyn ValueCandidates >) ;
    };
}

SubcommandCandidates!();