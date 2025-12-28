macro_rules! deps {
    () => {
        CompletionCandidate!();
        ValueCandidates!();
    };
}

macro_rules! ArgValueCandidates {
    () => {
        deps!();
        # [doc = " Extend [`Arg`][clap::Arg] with a [`ValueCandidates`]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use clap::Parser;"] # [doc = " use clap_complete::engine::{ArgValueCandidates, CompletionCandidate};"] # [doc = ""] # [doc = " #[derive(Debug, Parser)]"] # [doc = " struct Cli {"] # [doc = "     #[arg(long, add = ArgValueCandidates::new(|| { vec!["] # [doc = "         CompletionCandidate::new(\"foo\"),"] # [doc = "         CompletionCandidate::new(\"bar\"),"] # [doc = "         CompletionCandidate::new(\"baz\")] }))]"] # [doc = "     custom: Option<String>,"] # [doc = " }"] # [doc = " ```"] # [derive (Clone)] pub struct ArgValueCandidates (Arc < dyn ValueCandidates >) ;
    };
}

ArgValueCandidates!()