macro_rules! AsyncFnInTraitDiag {
    () => {
        pub (crate) struct AsyncFnInTraitDiag { pub sugg : Option < Vec < (Span , String) > > , }
    };
}

AsyncFnInTraitDiag!();