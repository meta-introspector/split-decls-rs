macro_rules! AmbiguousGlobImports {
    () => {
        pub (crate) struct AmbiguousGlobImports { pub ambiguity : AmbiguityErrorDiag , }
    };
}

AmbiguousGlobImports!()