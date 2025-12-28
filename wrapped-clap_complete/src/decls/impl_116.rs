macro_rules! deps {
    () => {
        CompletionCandidate!();
        ValueCandidates!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < F > ValueCandidates for F where F : Fn () -> Vec < CompletionCandidate > + Send + Sync , { fn candidates (& self) -> Vec < CompletionCandidate > { self () } }
    };
}

impl_116!()