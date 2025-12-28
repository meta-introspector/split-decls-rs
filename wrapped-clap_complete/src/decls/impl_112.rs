macro_rules! deps {
    () => {
        ValueCandidates!();
        SubcommandCandidates!();
        CompletionCandidate!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl SubcommandCandidates { # [doc = " Create a new `SubcommandCandidates` with a custom completer"] pub fn new < C > (completer : C) -> Self where C : ValueCandidates + 'static , { Self (Arc :: new (completer)) } # [doc = " All potential candidates for an external subcommand."] # [doc = ""] # [doc = " See [`CompletionCandidate`] for more information."] pub fn candidates (& self) -> Vec < CompletionCandidate > { self . 0 . candidates () } }
    };
}

impl_112!();