macro_rules! deps {
    () => {
        CompletionCandidate!();
        ValueCompleter!();
    };
}

macro_rules! ArgValueCompleter {
    () => {
        deps!();
        # [doc = " Extend [`Arg`][clap::Arg] with a completer"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use clap::Parser;"] # [doc = " use clap_complete::engine::{ArgValueCompleter, CompletionCandidate};"] # [doc = ""] # [doc = " fn custom_completer(current: &std::ffi::OsStr) -> Vec<CompletionCandidate> {"] # [doc = "     let mut completions = vec![];"] # [doc = "     let Some(current) = current.to_str() else {"] # [doc = "         return completions;"] # [doc = "     };"] # [doc = ""] # [doc = "     if \"foo\".starts_with(current) {"] # [doc = "         completions.push(CompletionCandidate::new(\"foo\"));"] # [doc = "     }"] # [doc = "     if \"bar\".starts_with(current) {"] # [doc = "         completions.push(CompletionCandidate::new(\"bar\"));"] # [doc = "     }"] # [doc = "     if \"baz\".starts_with(current) {"] # [doc = "         completions.push(CompletionCandidate::new(\"baz\"));"] # [doc = "     }"] # [doc = "     completions"] # [doc = " }"] # [doc = ""] # [doc = " #[derive(Debug, Parser)]"] # [doc = " struct Cli {"] # [doc = "     #[arg(long, add = ArgValueCompleter::new(custom_completer))]"] # [doc = "     custom: Option<String>,"] # [doc = " }"] # [doc = " ```"] # [derive (Clone)] pub struct ArgValueCompleter (Arc < dyn ValueCompleter >) ;
    };
}

ArgValueCompleter!()