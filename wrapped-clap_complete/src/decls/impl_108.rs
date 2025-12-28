macro_rules! deps {
    () => {
        ArgValueCandidates!();
        CompletionCandidate!();
        ValueCandidates!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl ArgValueCandidates { # [doc = " Create a new `ArgValueCandidates` with a custom completer"] pub fn new < C > (completer : C) -> Self where C : ValueCandidates + 'static , { Self (Arc :: new (completer)) } # [doc = " All potential candidates for an argument."] # [doc = ""] # [doc = " See [`CompletionCandidate`] for more information."] pub fn candidates (& self) -> Vec < CompletionCandidate > { self . 0 . candidates () } }
    };
}

impl_108!()