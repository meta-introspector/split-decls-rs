macro_rules! deps {
    () => {
        CompletionCandidate!();
        ArgValueCompleter!();
        ValueCompleter!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl ArgValueCompleter { # [doc = " Create a new `ArgValueCompleter` with a custom completer"] pub fn new < C > (completer : C) -> Self where C : ValueCompleter + 'static , { Self (Arc :: new (completer)) } # [doc = " Candidates that match `current`"] # [doc = ""] # [doc = " See [`CompletionCandidate`] for more information."] pub fn complete (& self , current : & OsStr) -> Vec < CompletionCandidate > { self . 0 . complete (current) } }
    };
}

impl_102!();