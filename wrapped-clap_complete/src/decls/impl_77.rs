macro_rules! deps {
    () => {
        CompletionCandidate!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < S : Into < OsString > > From < S > for CompletionCandidate { fn from (s : S) -> Self { CompletionCandidate :: new (s . into ()) } }
    };
}

impl_77!()