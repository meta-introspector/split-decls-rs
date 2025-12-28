macro_rules! deps {
    () => {
        ValueCompleter!();
        CompletionCandidate!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < F > ValueCompleter for F where F : Fn (& OsStr) -> Vec < CompletionCandidate > + Send + Sync , { fn complete (& self , current : & OsStr) -> Vec < CompletionCandidate > { self (current) } }
    };
}

impl_106!()